use super::*;

use std::collections::HashSet;

use components::*;
use resources::NpcMoves;

pub struct DoorOpenSystem;

#[derive(SystemData)]
pub struct DoorOpenSystemData<'a> {
    has_position: ReadStorage<'a, HasPosition>,
    door: WriteStorage<'a, Door>,
    opens_doors: ReadStorage<'a, OpensDoors>,
    char_render: WriteStorage<'a, CharRender>,

    blocks_airflow: WriteStorage<'a, BlocksAirflow>,
    blocks_movement: WriteStorage<'a, BlocksMovement>,
    blocks_visibility: WriteStorage<'a, BlocksVisibility>,

    npc_moves: Read<'a, NpcMoves>,
    entities: Entities<'a>,
}

impl<'a> System<'a> for DoorOpenSystem {
    type SystemData = DoorOpenSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        if !data.npc_moves.move_was_made {
            return;
        }

        // All the squares which have a DoorOpen component on them
        let mut has_door_open = HashSet::new();

        for (_, has_pos) in (&data.opens_doors, &data.has_position).join() {
            has_door_open.insert(has_pos.position);
        }

        for (mut door, has_pos, entity) in (&mut data.door, &data.has_position, &data.entities).join() {
            let has_adjacent = full_neighbors(has_pos.position).iter().any(|pos| has_door_open.contains(&pos));
            let renderable = data.char_render.get_mut(entity);

            let (should_open, should_close) = match door.door_behavior {
                DoorBehavior::FullAuto => (has_adjacent, !has_adjacent),
                DoorBehavior::AutoOpen => (has_adjacent, false),
                DoorBehavior::AutoClose => (false, !has_adjacent),
                DoorBehavior::StayOpen => (true, false),
                DoorBehavior::StayClosed => (false, true),
            };

            if should_open && should_close {
                // do nothing
            } else if should_open {
                door.door_state = components::DoorState::Open;
                data.blocks_visibility.remove(entity);
                data.blocks_airflow.remove(entity);
                data.blocks_movement.remove(entity);
                if let Some(renderable) = renderable {
                    renderable.disabled = true;
                }
            } else if should_close {
                door.door_state = components::DoorState::Closed;
                data.blocks_airflow
                    .insert(entity, components::BlocksAirflow)
                    .expect("The entity should be current");
                data.blocks_movement
                    .insert(entity, components::BlocksMovement)
                    .expect("The entity should be current");
                data.blocks_visibility
                    .insert(entity, components::BlocksVisibility)
                    .expect("The entity should be current");
                if let Some(renderable) = renderable {
                    renderable.disabled = false;
                }
            }
        }
    }
}
