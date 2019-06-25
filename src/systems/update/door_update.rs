use super::*;

use std::collections::HashSet;

use components::{BlocksAirflow, BlocksMovement, CharRender, Door, HasPosition, OpensDoors};

use world::TilePos;

pub struct DoorOpenSystem;

#[derive(SystemData)]
pub struct DoorOpenSystemData<'a> {
    has_position: ReadStorage<'a, HasPosition>,
    door: WriteStorage<'a, Door>,
    opens_doors: ReadStorage<'a, OpensDoors>,
    blocks_airflow: WriteStorage<'a, BlocksAirflow>,
    blocks_movement: WriteStorage<'a, BlocksMovement>,
    char_render: WriteStorage<'a, CharRender>,

    player_has_moved: Read<'a, PlayerHasMoved>,
    entities: Entities<'a>,
}

impl<'a> System<'a> for DoorOpenSystem {
    type SystemData = DoorOpenSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        if !data.player_has_moved.player_has_moved {
            return;
        }

        let mut has_door_open = HashSet::new();

        for (_, has_pos) in (&data.opens_doors, &data.has_position).join() {
            has_door_open.insert(has_pos.position);
        }

        for (mut door, has_pos, entity) in (&mut data.door, &data.has_position, &data.entities).join() {
            let should_open = neighbors(has_pos.position).into_iter().any(|pos| has_door_open.contains(&pos));

            if should_open {
                door.door_state = components::DoorState::Open;
                data.blocks_airflow.remove(entity);
                data.blocks_movement.remove(entity);
            } else {
                door.door_state = components::DoorState::Closed;
                data.blocks_airflow
                    .insert(entity, components::BlocksAirflow)
                    .expect("The entity should be current");
                data.blocks_movement
                    .insert(entity, components::BlocksMovement)
                    .expect("The entity should be current");
            }

            if let Some(renderable) = data.char_render.get_mut(entity) {
                *renderable = get_renderable(should_open);
            }
        }
    }
}

// TODO: pull this out into a helper function
fn neighbors(pos: TilePos) -> [TilePos; 9] {
    [
        pos,
        TilePos {
            x: pos.x - 1,
            y: pos.y - 1,
        },
        TilePos { x: pos.x - 1, y: pos.y },
        TilePos {
            x: pos.x - 1,
            y: pos.y + 1,
        },
        TilePos { x: pos.x, y: pos.y - 1 },
        TilePos { x: pos.x, y: pos.y + 1 },
        TilePos {
            x: pos.x + 1,
            y: pos.y - 1,
        },
        TilePos { x: pos.x + 1, y: pos.y },
        TilePos {
            x: pos.x + 1,
            y: pos.y + 1,
        },
    ]
}

fn get_renderable(open: bool) -> CharRender {
    if open {
        CharRender {
            glyph: ' ',
            fg_color: Color::WHITE,
        }
    } else {
        CharRender {
            glyph: 'd',
            fg_color: Color::WHITE,
        }
    }
}
