use super::*;

use std::collections::{BinaryHeap, HashMap};

use components::{BlocksAirflow, HasPosition, OxygenContainer, Vacuum};

use world::TilePos;

pub struct OxygenSpreadSystem;

#[derive(SystemData)]
pub struct OxygenSpreadSystemData<'a> {
    has_pos: ReadStorage<'a, HasPosition>,
    oxygen_cont: WriteStorage<'a, OxygenContainer>,
    vacuums: ReadStorage<'a, Vacuum>,
    blocks_airflow: ReadStorage<'a, BlocksAirflow>,

    entities: Entities<'a>,
    player_has_moved: Read<'a, PlayerHasMoved>,
}

// We do more iterations, with higher capacity, to make oxygen dispersal more "smooth"
const NUM_ITERATIONS: usize = 5;

struct OxygenTaker<'b> {
    container: &'b mut OxygenContainer,
    has_pos: &'b HasPosition,
    is_vacuum: bool,
}

impl<'b> Ord for OxygenTaker<'b> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.is_vacuum
            .cmp(&other.is_vacuum)
            .then(self.container.contents.cmp(&other.container.contents).reverse())
    }
}

impl<'b> PartialOrd for OxygenTaker<'b> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            self.is_vacuum
                .cmp(&other.is_vacuum)
                .then(self.container.contents.cmp(&other.container.contents).reverse()),
        )
    }
}

impl<'b> Eq for OxygenTaker<'b> {}

impl<'b> PartialEq for OxygenTaker<'b> {
    fn eq(&self, other: &Self) -> bool {
        self.container.contents == other.container.contents && self.is_vacuum == other.is_vacuum
    }
}

impl<'a> System<'a> for OxygenSpreadSystem {
    type SystemData = OxygenSpreadSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        if !data.player_has_moved.player_has_moved {
            return;
        }

        for _ in 0..NUM_ITERATIONS {
            // first, loop through all containers and see if they can share with their neighbors
            let mut oxygen_taker_queue = BinaryHeap::new();
            let mut oxygen_sharing = HashMap::new();

            for (ent, mut giver_oxygen, has_pos, _) in (&data.entities, &mut data.oxygen_cont, &data.has_pos, !&data.blocks_airflow).join()
            {
                if giver_oxygen.contents > 0 {
                    // NB: we only insert if capacity is >0; we will maintain this invariant
                    *oxygen_sharing.entry(has_pos.position).or_insert(0) += 1;
                    giver_oxygen.contents -= 1;
                }

                let is_vacuum = data.vacuums.get(ent).is_some();

                oxygen_taker_queue.push(OxygenTaker {
                    container: giver_oxygen,
                    has_pos,
                    is_vacuum,
                });
            }

            // then, loop through all containers and see if they can take some oxygen from a neighbor
            // basically we want to seek equillibrium, so lowest-contents containers take first,
            // then they're put back on the queue with their new priority. An object is only taken off the
            // queue if either (a) it's full, or (b) none of its neighbor plots have available oxygen
            loop {
                let taker = oxygen_taker_queue.pop();
                if taker.is_none() {
                    break;
                }
                let taker = taker.unwrap();

                for neighbor_pos in neighbors(taker.has_pos.position).iter() {
                    let neighbor_capacity = oxygen_sharing.get_mut(neighbor_pos);
                    if neighbor_capacity.is_none() {
                        continue;
                    }

                    let neighbor_capacity = neighbor_capacity.unwrap();
                    *neighbor_capacity -= 1;
                    taker.container.contents += 1;

                    if *neighbor_capacity <= 0 {
                        oxygen_sharing.remove(&neighbor_pos);
                    }

                    if taker.container.contents < taker.container.capacity {
                        oxygen_taker_queue.push(taker);
                    }

                    break;
                }
            }

            // then, all vacuums must vent their air into space
            for (ox, _, _) in (&mut data.oxygen_cont, &data.vacuums, !&data.blocks_airflow).join() {
                ox.contents = 0;
            }
        }
    }
}

fn neighbors(pos: TilePos) -> [TilePos; 5] {
    [
        pos,
        TilePos { x: pos.x - 1, y: pos.y },
        TilePos { x: pos.x, y: pos.y - 1 },
        TilePos { x: pos.x + 1, y: pos.y },
        TilePos { x: pos.x, y: pos.y + 1 },
    ]
}
