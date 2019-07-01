use super::*;

use std::collections::{BinaryHeap, HashMap, HashSet};

use components::{BlocksAirflow, HasPosition, OxygenContainer, Vacuum};
use resources::NpcMoves;

pub struct OxygenSpreadSystem;

#[derive(SystemData)]
pub struct OxygenSpreadSystemData<'a> {
    has_pos: ReadStorage<'a, HasPosition>,
    oxygen_cont: WriteStorage<'a, OxygenContainer>,
    vacuums: ReadStorage<'a, Vacuum>,
    blocks_airflow: ReadStorage<'a, BlocksAirflow>,

    npc_moves: Read<'a, NpcMoves>,
}

struct OxygenTaker<'b> {
    container: &'b mut OxygenContainer,
    has_pos: &'b HasPosition,
}

impl<'b> Ord for OxygenTaker<'b> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.container.contents.cmp(&other.container.contents).reverse()
    }
}

impl<'b> PartialOrd for OxygenTaker<'b> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.container.contents.cmp(&other.container.contents).reverse())
    }
}

impl<'b> Eq for OxygenTaker<'b> {}

impl<'b> PartialEq for OxygenTaker<'b> {
    fn eq(&self, other: &Self) -> bool {
        self.container.contents == other.container.contents
    }
}

impl<'a> System<'a> for OxygenSpreadSystem {
    type SystemData = OxygenSpreadSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        if !data.npc_moves.move_was_made {
            return;
        }

        let airblocks = timed!("Computing air blocks", {
            (&data.has_pos, &data.blocks_airflow)
                .join()
                .map(|(hp, _)| hp.position)
                .collect::<HashSet<TilePos>>()
        });

        let vacuums = timed!("Computing vacuum places", {
            (&data.has_pos, &data.vacuums)
                .join()
                .map(|(hp, _)| hp.position)
                .filter(|pos| !airblocks.contains(pos))
                .collect::<HashSet<TilePos>>()
        });

        let containers = timed!("Computing container locations", {
            (&data.has_pos, &data.oxygen_cont)
                .join()
                .map(|(hp, _)| hp.position)
                .filter(|pos| !airblocks.contains(pos))
                .collect::<HashSet<TilePos>>()
        });

        // First, all vacuums must vent their air into space
        for (ox, _, _) in (&mut data.oxygen_cont, &data.vacuums, !&data.blocks_airflow).join() {
            ox.contents = 0;
        }

        // Then; if a position is adjacent to a vacuum, all oxygen "shared" with that position will
        // just disappear (into the vacuum in theory, although it just goes into space; vacuums never
        // accumulate oxygen)
        let pos_should_vent = |pos| is_vacuum_adjacent(pos, &airblocks, &vacuums, &containers);

        // We do several small iterations per timestep, which smooths out the airflow
        for _ in 0..constants::oxygen::OXYGEN_SYSTEM_ITERATIONS {
            let mut oxygen_taker_queue = BinaryHeap::new();
            let mut oxygen_sharing = HashMap::new();

            // First, each oxygen container (which isn't already a vacuum) "shares" a small portion of
            // its available oxygen into a pot which anything adjacent can reach
            for (mut giver_oxygen, has_pos, _) in (&mut data.oxygen_cont, &data.has_pos, !&data.vacuums)
                .join()
                .filter(|(_, hp, _)| !airblocks.contains(&hp.position))
            {
                if giver_oxygen.contents > 0 {
                    let reduction = std::cmp::min(giver_oxygen.contents, constants::oxygen::OXYGEN_SYSTEM_SHARE_PER_ITERATION);
                    giver_oxygen.contents -= reduction;

                    // If it's adjacent to a vacuum that "shared" oxygen just goes away
                    if !pos_should_vent(has_pos.position) {
                        *oxygen_sharing.entry(has_pos.position).or_insert(0) += reduction;
                    }
                }

                oxygen_taker_queue.push(OxygenTaker {
                    container: giver_oxygen,
                    has_pos,
                });
            }

            // Then, each container (in priority order based on need, that is, low oxygen) "takes" one
            // unit of oxygen from an adjacent square, if available, then goes back on the queue. A container
            // pops off the queue when either it's full, or there is nothing adjacent to take.

            // Worst case this is O(SHARING_PER_ITERATION * Nlog(N)), where N is number of oxygen containers,
            // since we use a min heap for priority queue
            loop {
                let taker = oxygen_taker_queue.pop();
                if taker.is_none() {
                    break;
                }
                let taker = taker.unwrap();

                for neighbor_pos in full_neighbors(taker.has_pos.position).iter() {
                    let neighbor_capacity = oxygen_sharing.get_mut(neighbor_pos);
                    if neighbor_capacity.is_none() {
                        continue;
                    }

                    let neighbor_capacity = neighbor_capacity.unwrap();
                    *neighbor_capacity -= 1;
                    taker.container.contents += 1;

                    if *neighbor_capacity == 0 {
                        oxygen_sharing.remove(&neighbor_pos);
                    }

                    if taker.container.contents < taker.container.capacity {
                        oxygen_taker_queue.push(taker);
                    }

                    break;
                }
            }
        }
    }
}

fn is_vacuum_adjacent(pos: TilePos, blocks: &HashSet<TilePos>, known_vacuums: &HashSet<TilePos>, containers: &HashSet<TilePos>) -> bool {
    for neighbor_pos in full_neighbors(pos).iter() {
        if blocks.contains(neighbor_pos) {
            continue;
        }
        if known_vacuums.contains(neighbor_pos) {
            return true;
        }
        if !containers.contains(neighbor_pos) {
            return true;
        }
    }
    false
}
