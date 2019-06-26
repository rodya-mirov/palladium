use super::*;

use resources::NpcMoves;

pub struct PlayerNotMoved;

impl<'a> System<'a> for PlayerNotMoved {
    type SystemData = Write<'a, NpcMoves>;

    fn run(&mut self, mut data: Self::SystemData) {
        turn_state_helpers::timestep(&mut data);
    }
}
