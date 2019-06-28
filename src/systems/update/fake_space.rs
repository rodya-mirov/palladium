//! System for inserting "fake" "space tiles"

use super::*;

use components::{Camera, CharRender, HasPosition, ImaginaryVisibleTile, Visible, ZLevel};
use resources::NpcMoves;

use world::{TilePos, VisibilityType};

use camera_helpers::get_camera_bounds;

#[derive(SystemData)]
pub struct FakeSpaceInserterSystemData<'a> {
    has_position: ReadStorage<'a, HasPosition>,
    camera: ReadStorage<'a, Camera>,
    entities: Entities<'a>,
    imaginaries: ReadStorage<'a, ImaginaryVisibleTile>,
    lazy_update: Read<'a, LazyUpdate>,

    npc_moves: Read<'a, NpcMoves>,
}

pub struct FakeSpaceInserterSystem;

impl<'a> System<'a> for FakeSpaceInserterSystem {
    type SystemData = FakeSpaceInserterSystemData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        if !data.npc_moves.move_was_made {
            return;
        }

        let to_delete: Vec<Entity> = (&data.entities, &data.imaginaries).join().map(|(ent, _)| ent).collect();
        for ent in to_delete {
            data.lazy_update
                .exec_mut(move |world| world.delete_entity(ent).expect("This is a current entity so shouldn't be invalid"));
        }

        let camera_bounds = (&data.camera, &data.has_position)
            .join()
            .map(|(&camera, &has_position)| get_camera_bounds(has_position, camera))
            .next()
            .expect("Camera should exist and have a position");

        for x in camera_bounds.x_min..=camera_bounds.x_max {
            for y in camera_bounds.y_min..=camera_bounds.y_max {
                data.lazy_update
                    .create_entity(&data.entities)
                    .with(HasPosition {
                        position: TilePos { x, y },
                    })
                    .with(Visible {
                        visibility: VisibilityType::NotSeen,
                        memorable: false,
                    })
                    .with(CharRender {
                        bg_color: Color::BLACK,
                        fg_color: Color::BLACK,
                        glyph: ' ',
                        z_level: ZLevel::NegativeInfinity,
                    })
                    .build();
            }
        }
    }
}
