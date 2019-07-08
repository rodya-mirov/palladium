use super::*;

use resources::{GameClock, NpcMoves, RenderStale};

pub struct PlayerNotMoved;

impl<'a> System<'a> for PlayerNotMoved {
    type SystemData = (Write<'a, NpcMoves>, Write<'a, GameClock>, Write<'a, RenderStale>);

    fn run(&mut self, mut data: Self::SystemData) {
        turn_state_helpers::timestep(&mut data.0, &mut data.1, &mut data.2);
    }
}
