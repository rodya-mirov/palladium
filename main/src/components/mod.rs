use super::*;

use quicksilver::graphics::Color;
use serde::{Deserialize, Serialize};
use specs::{
    saveload::{SimpleMarker, SimpleMarkerAllocator},
    storage::{DenseVecStorage, HashMapStorage, NullStorage, VecStorage},
};

use world::{TilePos, VisibilityType};

pub type SaveComponent = SimpleMarker<()>;
pub type SaveComponentAllocator = SimpleMarkerAllocator<()>;

#[derive(Component, Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct HasPosition {
    pub position: TilePos,
}

// Used to represent "deep space" -- but these are deleted and recreated every timestep
// so need to be visible && renderable && have a position but aren't really meaningful
// from a game system perspective; space is everywhere but these only exist in the camera zone
#[derive(Component, Default, Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[storage(NullStorage)]
pub struct ImaginaryVisibleTile;

#[derive(Component, Debug, Copy, Clone, Eq, PartialEq, Default, Serialize, Deserialize)]
#[storage(NullStorage)]
pub struct BlocksMovement; // I mean, it's direct

#[derive(Component, Debug, Copy, Clone, Eq, PartialEq, Default, Serialize, Deserialize)]
#[storage(NullStorage)]
pub struct BlocksVisibility; // I mean, it's direct

#[derive(Component, Debug, Copy, Clone, Eq, PartialEq, Default, Serialize, Deserialize)]
#[storage(NullStorage)]
pub struct BlocksAirflow; // this is for walls and doors and stuff

#[derive(Component, Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct Visible {
    // whether the object is currently visible
    pub visibility: VisibilityType,
    // If true, the object is marked as "remembered" if you saw it once, but can't anymore
    // This should not be used for objects that move
    // TODO: probably replace this with a "memory" system (memory entities; so if it's deleted
    // in your absence, you won't know until you see it again)
    pub memorable: bool,
}

#[derive(Debug, Component, Clone, Serialize, Deserialize)]
#[storage(HashMapStorage)]
pub struct Hackable {
    // name, as it appears on the hackable menu; probably not unique
    // TODO: why can't this be a &'static str? Something about deriving de/serialize not work.
    pub name: String,
    pub hack_state: HackState,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum HackState {
    Uncompromised,
    Compromised,
}

#[derive(Component, Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct CharRender {
    pub glyph: char,
    pub disabled: bool,
    pub z_level: ZLevel,
    pub bg_color: Color, // use a:0 for no background
    pub fg_color: Color,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ZLevel {
    // easy enough to add more here; these are only used for sorting
    // this is just sorting for drawing, not spatial z-level (z-coordinate)
    NegativeInfinity,
    Floor,
    OnFloor,
}

impl ZLevel {
    fn to_num(self) -> i32 {
        match self {
            ZLevel::NegativeInfinity => -5,
            ZLevel::Floor => 0,
            ZLevel::OnFloor => 1,
        }
    }
}

impl Ord for ZLevel {
    fn cmp(&self, other: &ZLevel) -> std::cmp::Ordering {
        self.to_num().cmp(&other.to_num())
    }
}

impl PartialOrd for ZLevel {
    fn partial_cmp(&self, other: &ZLevel) -> Option<std::cmp::Ordering> {
        Some(self.to_num().cmp(&other.to_num()))
    }
}

#[derive(Component, Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[storage(HashMapStorage)]
pub struct Player {
    // nothing i guess? Probably something later
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
#[storage(HashMapStorage)]
pub struct Talkable {
    pub name: String,
    // TODO: dialogue trees and stuff, it's gonna get complicated but this is just a placeholder rn
}

#[derive(Component, Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[storage(HashMapStorage)]
pub enum NPC {
    Alien(AlienAI),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum AlienAI {
    Wander,
}

#[derive(Component, Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[storage(DenseVecStorage)]
pub struct OxygenContainer {
    pub capacity: usize, // how much air the entity can hold
    pub contents: usize, // how much air the entity currently has
}

#[derive(Component, Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[storage(HashMapStorage)]
pub struct Breathes {
    pub capacity: usize,
    pub contents: usize,
    // +2 per tick if there is at least this much air in their tile
    pub fast_gain_threshold: usize,
    // +1 per tick if there is at least this much air in their tile
    pub slow_gain_threshold: usize,
    // -1 per tick if there is at least this much air in their tile
    pub slow_drop_threshold: usize,
    // -2 per tick if the above is not satisfied
}

impl Default for Breathes {
    fn default() -> Self {
        Breathes {
            capacity: constants::oxygen::DEFAULT_FULL_OXYGEN,
            contents: constants::oxygen::DEFAULT_FULL_OXYGEN,
            fast_gain_threshold: constants::oxygen::FAST_GAIN_THRESHOLD,
            slow_gain_threshold: constants::oxygen::SLOW_GAIN_THRESHOLD,
            slow_drop_threshold: constants::oxygen::SLOW_DROP_THRESHOLD,
        }
    }
}

#[derive(Component, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[storage(HashMapStorage)]
pub enum CanSuffocate {
    // enum determining the behavior of what happens when they _do_ suffocate
    // Player: triggers player death things
    Player,
    // something else: just dies (entity is deleted)
    Death,
}

#[derive(Component, Debug, Copy, Clone, Eq, PartialEq, Default, Serialize, Deserialize)]
#[storage(NullStorage)]
pub struct Vacuum; // oxygen container which destroys all its oxygen each timestep

#[derive(Component, Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[storage(HashMapStorage)]
pub struct Door {
    pub door_state: DoorState,
    pub door_behavior: DoorBehavior,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum DoorState {
    Open,
    Closed,
}

#[allow(dead_code)] // Leaving these in as I sort of want to use them later, but not yet
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum DoorBehavior {
    FullAuto,
    AutoOpen,
    AutoClose,
    StayClosed,
    StayOpen,
}

#[derive(Component, Debug, Copy, Clone, Eq, PartialEq, Default, Serialize, Deserialize)]
#[storage(NullStorage)]
pub struct OpensDoors;

#[derive(Component, Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[storage(HashMapStorage)]
pub struct Camera {
    // actual dimension is 2*rad + 1
    pub x_rad: usize,
    pub y_rad: usize,
}
