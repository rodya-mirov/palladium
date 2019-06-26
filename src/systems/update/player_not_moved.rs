use super::*;

use resources::PlayerHasMoved;

pub struct PlayerNotMoved;

impl<'a> System<'a> for PlayerNotMoved {
    type SystemData = Write<'a, PlayerHasMoved>;

    fn run(&mut self, mut data: Self::SystemData) {
        data.player_has_moved = false;
    }
}
