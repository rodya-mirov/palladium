use super::*;

use std::ops::{Add, AddAssign};

mod map;

pub use map::{Map, MapGenerationParams, SquareType, VisibilityType};

#[derive(Clone, Debug)]
pub struct WorldState {
    pub map: Map,
}

impl WorldState {
    pub fn new(map: Map) -> Self {
        WorldState { map }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct TilePos {
    pub x: i32,
    pub y: i32,
}

impl Add<TilePos> for TilePos {
    type Output = TilePos;

    fn add(self, rhs: TilePos) -> TilePos {
        TilePos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<TilePos> for TilePos {
    fn add_assign(&mut self, rhs: TilePos) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
