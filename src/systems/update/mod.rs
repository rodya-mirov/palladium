//! Module for systems which are intended to be run every update frame, as opposed to only in frames when
//! the appropriate panel has keyboard focus

use super::*;

mod player_not_moved;
pub use player_not_moved::PlayerNotMoved;

mod visibility;
pub use visibility::VisibilitySystem;

mod oxygen_spread;
pub use oxygen_spread::OxygenSpreadSystem;

mod door_update;
pub use door_update::DoorOpenSystem;

mod fake_space;
pub use fake_space::FakeSpaceInserterSystem;
