use super::*;

use specs::{Read, ReadExpect, ReadStorage, System, Write, WriteStorage};

use world::TilePos;

mod dialogue_helpers;

mod input;
mod render;
mod update;

pub use input::*;
pub use render::*;
pub use update::*;

fn direct_neighbors(pos: TilePos) -> [TilePos; 5] {
    [
        pos,
        TilePos { x: pos.x - 1, y: pos.y },
        TilePos { x: pos.x, y: pos.y - 1 },
        TilePos { x: pos.x, y: pos.y + 1 },
        TilePos { x: pos.x + 1, y: pos.y },
    ]
}

fn full_neighbors(pos: TilePos) -> [TilePos; 9] {
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
