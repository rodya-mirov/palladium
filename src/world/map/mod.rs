use super::*;

use specs::Builder;

mod params;
mod rand_gen;

pub use params::MapGenerationParams;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum GenSquareType {
    Floor,
    Wall,
    Open,
}

fn to_real_square(kind: GenSquareType) -> Option<SquareType> {
    match kind {
        GenSquareType::Floor => Some(SquareType::Floor),
        GenSquareType::Wall => Some(SquareType::Wall),
        GenSquareType::Open => None,
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum SquareType {
    Floor,
    Wall,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum VisibilityType {
    NotSeen,
    Remembered,
    CurrentlyVisible,
}

#[derive(Clone, Debug)]
pub struct Map {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,

    row_width: usize,
    col_height: usize,
}

fn make_glyph(kind: SquareType) -> char {
    match kind {
        SquareType::Floor => ' ',
        SquareType::Wall => '█',
    }
}

fn make_bg_color(kind: SquareType) -> Color {
    match kind {
        SquareType::Floor => Color {
            r: 0.4,
            g: 0.6,
            b: 0.5,
            a: 1.0,
        },
        SquareType::Wall => Color {
            r: 0.4,
            g: 0.6,
            b: 0.5,
            a: 1.0,
        },
    }
}

fn make_fg_color(kind: SquareType) -> Color {
    match kind {
        SquareType::Floor => Color::WHITE,
        SquareType::Wall => Color::WHITE,
    }
}

fn get_occludes(kind: SquareType) -> bool {
    match kind {
        SquareType::Floor => false,
        SquareType::Wall => true,
    }
}

fn get_blocks(kind: SquareType) -> bool {
    match kind {
        SquareType::Floor => false,
        SquareType::Wall => true,
    }
}

fn get_blocks_airflow(kind: SquareType) -> bool {
    match kind {
        SquareType::Floor => false,
        SquareType::Wall => true,
    }
}

fn get_vacuum(kind: SquareType) -> bool {
    match kind {
        SquareType::Floor => false,
        SquareType::Wall => false,
    }
}

impl Map {
    pub fn make_random(params: &MapGenerationParams, world: &mut World) -> Self {
        let gen_result = rand_gen::rand_gen(params);

        let row_width = gen_result.width;
        let col_height = gen_result.height;

        let x_min = 0 as i32;
        let x_max = (row_width - 1) as i32;

        let y_min = 0 as i32;
        let y_max = (col_height - 1) as i32;

        let mut x = x_min;
        let mut y = y_min;

        let num_tiles = gen_result.cells.len();

        if num_tiles != row_width * col_height {
            panic!(
                "MapGen: Misconfigured dimensions: got width={}, height={}, but num_tiles={}",
                row_width, col_height, num_tiles
            );
        }

        for square in gen_result.cells {
            if let Some(kind) = to_real_square(square.square_type) {
                let mut tile_builder = world
                    .create_entity()
                    .with(components::Visible {
                        visibility: VisibilityType::NotSeen,
                        memorable: true,
                    })
                    .with(components::HasPosition {
                        position: TilePos { x, y },
                    })
                    .with(components::OxygenContainer {
                        capacity: 100,
                        contents: 100,
                    })
                    .with(components::CharRender {
                        glyph: make_glyph(kind),
                        z_level: components::ZLevel::Floor,
                        bg_color: make_bg_color(kind),
                        fg_color: make_fg_color(kind),
                    });

                if get_occludes(kind) {
                    tile_builder = tile_builder.with(components::BlocksVisibility);
                }
                if get_blocks(kind) {
                    tile_builder = tile_builder.with(components::BlocksMovement);
                }
                if get_blocks_airflow(kind) {
                    tile_builder = tile_builder.with(components::BlocksAirflow);
                }
                if get_vacuum(kind) {
                    tile_builder = tile_builder.with(components::Vacuum);
                }

                let _tile = tile_builder.build();
            }

            x += 1;
            if x > x_max {
                x = x_min;
                y += 1;
            }
        }

        for other in gen_result.others {
            match other {
                rand_gen::GeneratedEntity::Door(pos) => {
                    world
                        .create_entity()
                        .with(components::HasPosition { position: pos })
                        .with(components::Hackable {
                            name: "Door",
                            hack_state: components::HackState::Door(components::DoorHackState::Uncompromised),
                        })
                        .with(components::CharRender {
                            glyph: 'd',
                            z_level: components::ZLevel::OnFloor,
                            bg_color: CLEAR,
                            fg_color: Color::WHITE,
                        })
                        .with(components::Visible {
                            visibility: VisibilityType::NotSeen,
                            memorable: true,
                        })
                        .with(components::BlocksVisibility)
                        .with(components::Door {
                            door_state: components::DoorState::Closed,
                        })
                        .with(components::BlocksAirflow)
                        .with(components::BlocksMovement)
                        .build();
                }
                rand_gen::GeneratedEntity::Airlock(pos) => {
                    world
                        .create_entity()
                        .with(components::HasPosition { position: pos })
                        .with(components::Hackable {
                            name: "Airlock",
                            hack_state: components::HackState::Door(components::DoorHackState::CompromisedShut),
                        })
                        .with(components::CharRender {
                            glyph: 'A',
                            z_level: components::ZLevel::OnFloor,
                            bg_color: CLEAR,
                            fg_color: Color::WHITE,
                        })
                        .with(components::Visible {
                            visibility: VisibilityType::NotSeen,
                            memorable: true,
                        })
                        .with(components::BlocksVisibility)
                        .with(components::Door {
                            door_state: components::DoorState::Closed,
                        })
                        .with(components::BlocksAirflow)
                        .with(components::BlocksMovement)
                        .build();
                }
                rand_gen::GeneratedEntity::Rubbish(pos) => {
                    world
                        .create_entity()
                        .with(components::HasPosition { position: pos })
                        .with(components::CharRender {
                            glyph: '`',
                            z_level: components::ZLevel::OnFloor,
                            bg_color: CLEAR,
                            fg_color: quicksilver::graphics::Color {
                                r: 0.7,
                                g: 0.7,
                                b: 0.7,
                                a: 1.0,
                            },
                        })
                        .with(components::Visible {
                            visibility: VisibilityType::NotSeen,
                            memorable: false,
                        })
                        .with(components::BlocksMovement)
                        .build();
                }
                rand_gen::GeneratedEntity::Pillar(pos) => {
                    world
                        .create_entity()
                        .with(components::HasPosition { position: pos })
                        .with(components::CharRender {
                            glyph: 'I',
                            z_level: components::ZLevel::OnFloor,
                            bg_color: CLEAR,
                            fg_color: quicksilver::graphics::Color {
                                r: 0.8,
                                g: 0.6,
                                b: 1.0,
                                a: 1.0,
                            },
                        })
                        .with(components::Visible {
                            visibility: VisibilityType::NotSeen,
                            memorable: false,
                        })
                        .with(components::BlocksVisibility)
                        .with(components::BlocksMovement)
                        .build();
                }
            }
        }

        Map {
            x_min,
            x_max,
            y_min,
            y_max,
            row_width,
            col_height,
        }
    }
}
