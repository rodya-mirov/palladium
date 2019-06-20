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
    // If true, the object is marked as "remembered" if you saw it once, but can't anymore
    // This should not be used for objects that move
    // TODO: probably replace this with a "memory" system (memory entities; so if it's delete
    // in your absence, you won't know until you see it again)
    pub memorable: bool,
    // whether the object blocks visibility
    // TODO: should we pull occludes off into its own component with a NullStorage?
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
