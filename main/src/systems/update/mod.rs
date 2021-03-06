//! Module for systems which are intended to be run every update frame, as opposed to only in frames when
//! the appropriate panel has keyboard focus

use super::*;

mod upkeep;
pub use upkeep::{CallbackCheckerSystem, DialogueUpdateSystem, GameIsQuitCheckerSystem, PlayerNotMoved};

mod visibility;
pub use visibility::VisibilitySystem;

mod oxygen_spread;
pub use oxygen_spread::OxygenSpreadSystem;

mod door_update;
pub use door_update::DoorOpenSystem;

mod fake_space;
pub use fake_space::FakeSpaceInserterSystem;

mod breathe;
pub use breathe::BreatheSystem;

mod npc_move;
pub use npc_move::NpcMoveSystem;

mod saves;
pub use saves::{DeserializeSystem, SerializeSystem};
