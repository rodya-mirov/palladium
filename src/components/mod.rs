use super::*;

use quicksilver::graphics::Color;
use specs::storage::{DenseVecStorage, HashMapStorage, NullStorage, VecStorage};

use world::{TilePos, VisibilityType};

#[derive(Component, Debug, Copy, Clone, Eq, PartialEq)]
#[storage(VecStorage)]
pub struct HasPosition {
    pub position: TilePos,
}

#[derive(Component, Debug, Copy, Clone, Eq, PartialEq, Default)]
#[storage(NullStorage)]
pub struct BlocksMovement; // I mean, it's direct

#[derive(Component, Debug, Copy, Clone, Eq, PartialEq, Default)]
#[storage(NullStorage)]
pub struct BlocksAirflow; // walls and doors and stuff

#[derive(Component, Debug, Copy, Clone, Eq, PartialEq, Default)]
#[storage(NullStorage)]
pub struct MapTile; // Only used to mark "this is a map tile, so it's cached and stuff" for efficiency

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
#[storage(DenseVecStorage)]
pub struct OxygenContainer {
    pub capacity: usize, // how much air the entity can hold
    pub contents: usize, // how much air the entity currently has
}

#[derive(Component, Debug, Copy, Clone, Eq, PartialEq, Default)]
#[storage(NullStorage)]
pub struct Vacuum; // oxygen container which destroys all its oxygen each timestep

#[derive(Component, Debug, Copy, Clone, Eq, PartialEq)]
#[storage(DenseVecStorage)]
pub struct NeedsOxygen {
    pub capacity: usize,          // how much air the "lungs" can "hold"
    pub contents: usize,          // how much air is in the "lungs"
    pub breathing_speed: usize,   // in oxygen, gains this much / timestep
    pub consumption_speed: usize, // in oxygen, depletes this much / timestep from enviroment
    pub depletion_rate: usize,    // without oxygen, how fast contents deplete
}

#[derive(Component, Debug, Copy, Clone, Eq, PartialEq)]
#[storage(HashMapStorage)]
pub struct Camera {
    // actual dimension is 2*rad + 1
    pub x_rad: usize,
    pub y_rad: usize,
}
