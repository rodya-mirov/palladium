use super::*;

use std::collections::HashSet;

use components::*;
use resources::NpcMoves;

pub struct DoorOpenSystem;

#[derive(SystemData)]
pub struct DoorOpenSystemData<'a> {
    has_position: ReadStorage<'a, HasPosition>,
    door: WriteStorage<'a, Door>,
    hackable: ReadStorage<'a, Hackable>,
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

        let mut has_door_open = HashSet::new();

        for (_, has_pos) in (&data.opens_doors, &data.has_position).join() {
            has_door_open.insert(has_pos.position);
        }

        for (mut door, has_pos, entity) in (&mut data.door, &data.has_position, &data.entities).join() {
            let mut should_open = full_neighbors(has_pos.position).iter().any(|pos| has_door_open.contains(&pos));
            let mut should_close = !should_open;

            if let Some(hackable) = &data.hackable.get(entity) {
                let HackState::Door(door_hack_state) = &hackable.hack_state;
                match door_hack_state {
                    DoorHackState::CompromisedNormal => {}
                    DoorHackState::Uncompromised => {}
                    DoorHackState::CompromisedOpen => {
                        should_open = true;
                        should_close = false;
                    }
                    DoorHackState::CompromisedShut => {
                        should_open = false;
                        should_close = true;
                    }
                }
            }

            if should_open {
                door.door_state = components::DoorState::Open;
                data.blocks_visibility.remove(entity);
                data.blocks_airflow.remove(entity);
                data.blocks_movement.remove(entity);
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
            }

            if let Some(renderable) = data.char_render.get_mut(entity) {
                if should_open {
                    renderable.fg_color = CLEAR;
                } else {
                    renderable.fg_color = Color::WHITE;
                }
            }
        }
    }
}
