use super::*;

use quicksilver::graphics::Color;
use specs::storage::{HashMapStorage, NullStorage, VecStorage};

use world::{SquareType, TilePos, VisibilityType};

#[derive(Component, Debug, Copy, Clone, Eq, PartialEq)]
#[storage(VecStorage)]
pub struct HasPosition {
    pub position: TilePos,
}

#[derive(Component, Debug, Copy, Clone, Eq, PartialEq, Default)]
#[storage(NullStorage)]
pub struct BlocksMovement; // I mean, it's direct

#[derive(Component, Debug, Copy, Clone, Eq, PartialEq)]
#[storage(VecStorage)]
pub struct MapTile {
    pub kind: SquareType,
}

#[derive(Component, Debug, Copy, Clone, Eq, PartialEq)]
#[storage(VecStorage)]
pub struct Visible {
    // whether the object is currently visible
    pub visibility: VisibilityType,
    // TODO: should we pull occludes off into its own component with a NullStorage?
    // whether the object blocks visibility
    pub occludes: bool,
}

#[derive(Component, Debug, Copy, Clone, PartialEq)]
#[storage(VecStorage)]
pub struct CharRender {
    pub glyph: char,
    pub fg_color: Color,
}

#[derive(Component, Debug, Copy, Clone, Eq, PartialEq)]
#[storage(HashMapStorage)]
pub struct Player {
    // nothing i guess? Probably something later
}

#[derive(Component, Debug, Copy, Clone, Eq, PartialEq)]
#[storage(HashMapStorage)]
pub struct Camera {
    // actual dimension is 2*rad + 1
    pub x_rad: usize,
    pub y_rad: usize,
}
