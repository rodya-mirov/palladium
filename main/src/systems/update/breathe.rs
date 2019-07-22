use super::*;

use std::collections::HashMap;

use components::{Breathes, CanSuffocate, HasPosition, OxygenContainer};
use resources::{Callbacks, NpcMoves};

fn safe_subtract(start: usize, subtraction: usize) -> usize {
    if start > subtraction {
        start - subtraction
    } else {
        0
    }
}

fn add_and_cap(start: usize, add: usize, cap: usize) -> usize {
    std::cmp::min(cap, start + add)
}

#[derive(SystemData)]
pub struct BreathSystemData<'a> {
    has_pos: ReadStorage<'a, HasPosition>,
    oxygen_cont: WriteStorage<'a, OxygenContainer>,
    breathes: WriteStorage<'a, Breathes>,
    can_suffocate: ReadStorage<'a, CanSuffocate>,
    entities: Entities<'a>,

    npc_moves: Read<'a, NpcMoves>,
    callbacks: Write<'a, Callbacks>,
}

pub struct BreatheSystem;

impl<'a> System<'a> for BreatheSystem {
    type SystemData = BreathSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        if !data.npc_moves.move_was_made {
            return;
        }

        // first build the amount of oxygen available at all places
        let mut oxygen_map: HashMap<TilePos, usize> = HashMap::new();
        for (hp, oc) in (&data.has_pos, &data.oxygen_cont).join() {
            *oxygen_map.entry(hp.position).or_insert(0) += oc.contents;
        }

        // then look through all the breathers and see what happens to them
        for (breathe, hp, entity) in (&mut data.breathes, &data.has_pos, &data.entities).join() {
            let oxygen_here = oxygen_map.get(&hp.position).copied().unwrap_or(0);

            if oxygen_here >= breathe.fast_gain_threshold {
                add_oxygen(breathe, constants::oxygen::FAST_GAIN_SPEED);
            } else if oxygen_here >= breathe.slow_gain_threshold {
                add_oxygen(breathe, constants::oxygen::SLOW_GAIN_SPEED);
            } else if oxygen_here >= breathe.slow_drop_threshold {
                lose_oxygen(breathe, constants::oxygen::SLOW_DROP_SPEED);
                if breathe.contents == 0 {
                    if let Some(cs) = data.can_suffocate.get(entity) {
                        process_suffocate(entity, cs, &mut data.callbacks, &data.entities);
                    }
                }
            } else {
                lose_oxygen(breathe, constants::oxygen::FAST_DROP_SPEED);
                if breathe.contents == 0 {
                    if let Some(cs) = data.can_suffocate.get(entity) {
                        process_suffocate(entity, cs, &mut data.callbacks, &data.entities);
                    }
                }
            }
        }
    }
}

fn add_oxygen(breathe: &mut Breathes, addition: usize) {
    breathe.contents = add_and_cap(breathe.contents, addition, breathe.capacity);
}

fn lose_oxygen(breathe: &mut Breathes, loss: usize) {
    breathe.contents = safe_subtract(breathe.contents, loss);
}

fn process_suffocate(entity: Entity, cs: &CanSuffocate, callbacks: &mut Callbacks, entities: &Entities) {
    use resources::Callback;

    match cs {
        CanSuffocate::Player => {
            let builder = dialogue_helpers::DialogueBuilder::new(
                "You have been without air for too long.\n\nThis life is over, but the tether pulls you back.",
            )
            .with_option("[Continue]", vec![Callback::EndDialogue, Callback::LoadGame]);

            dialogue_helpers::launch_dialogue(builder, callbacks);
        }
        CanSuffocate::Death => {
            // thing dies, uh, delete it
            // TODO: do something more interesting here?
            entities.delete(entity).expect("Entity should be live, since it just came up");
        }
    }
}
