use super::*;

use specs::{Builder, Entity, World};

mod params;
mod rand_gen;

pub use params::MapGenerationParams;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum SquareType {
    Open, // not allocated to anything (should not be accessible to the player)
    Floor,
    Wall,
    Door, // can walk, can't see through
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum VisibilityType {
    NotSeen,
    Remembered,
    CurrentlyVisible,
}

#[derive(Clone, Debug)]
pub struct Map {
    tiles: Vec<Entity>,

    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,

    row_width: usize,
    col_height: usize,
}

fn make_glyph(kind: SquareType) -> char {
    match kind {
        SquareType::Open => '*',
        SquareType::Floor => ' ',
        SquareType::Wall => '█',
        SquareType::Door => 'd',
    }
}

fn get_occludes(kind: SquareType) -> bool {
    match kind {
        SquareType::Open => false,

        SquareType::Floor => false,

        SquareType::Wall => true,
        SquareType::Door => true,
    }
}

fn get_blocks(kind: SquareType) -> bool {
    match kind {
        SquareType::Floor => false,
        SquareType::Door => false,

        SquareType::Wall => true,
        SquareType::Open => true,
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

        let mut tiles = Vec::with_capacity(row_width * col_height);
        let num_tiles = gen_result.cells.len();

        if num_tiles != row_width * col_height {
            panic!(
                "MapGen: Misconfigured dimensions: got width={}, height={}, but num_tiles={}",
                row_width, col_height, num_tiles
            );
        }

        for square in gen_result.cells {
            let mut tile_builder = world
                .create_entity()
                .with(components::MapTile { kind: square.square_type })
                .with(components::Visible {
                    visibility: VisibilityType::NotSeen,
                    occludes: get_occludes(square.square_type),
                    memorable: true,
                })
                .with(components::HasPosition {
                    position: TilePos { x, y },
                })
                .with(components::CharRender {
                    glyph: make_glyph(square.square_type),
                    fg_color: quicksilver::graphics::Color::WHITE,
                });

            if get_blocks(square.square_type) {
                tile_builder = tile_builder.with(components::BlocksMovement);
            }

            let tile = tile_builder.build();

            tiles.push(tile);

            x += 1;
            if x > x_max {
                x = x_min;
                y += 1;
            }
        }

        for other in gen_result.others {
            match other {
                rand_gen::GeneratedEntity::Rubbish(pos) => {
                    world
                        .create_entity()
                        .with(components::HasPosition { position: pos })
                        .with(components::CharRender {
                            glyph: '`',
                            fg_color: quicksilver::graphics::Color {
                                r: 0.7,
                                g: 0.7,
                                b: 0.7,
                                a: 1.0,
                            },
                        })
                        .with(components::Visible {
                            visibility: VisibilityType::NotSeen,
                            occludes: false,
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
                            fg_color: quicksilver::graphics::Color {
                                r: 0.8,
                                g: 0.6,
                                b: 1.0,
                                a: 1.0,
                            },
                        })
                        .with(components::Visible {
                            visibility: VisibilityType::NotSeen,
                            occludes: true,
                            memorable: false,
                        })
                        .with(components::BlocksMovement)
                        .build();
                }
            }
        }

        Map {
            tiles,
            x_min,
            x_max,
            y_min,
            y_max,
            row_width,
            col_height,
        }
    }

    pub fn get_tile(&self, tile_pos: TilePos) -> Option<Entity> {
        let (x, y) = (tile_pos.x, tile_pos.y);

        if x < self.x_min || x > self.x_max || y < self.y_min || y > self.y_max {
            return None;
        }

        let index = self.get_index(x, y);
        self.tiles.get(index).map(|&entity| entity)
    }

    fn get_index(&self, x: i32, y: i32) -> usize {
        let xoff = (x - self.x_min) as usize;
        let yoff = (y - self.y_min) as usize;

        xoff + self.row_width * yoff
    }
}
