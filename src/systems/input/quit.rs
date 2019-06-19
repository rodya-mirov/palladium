use super::*;

use panel::GameIsQuit;

pub struct QuitSystem {}

impl<'a> System<'a> for QuitSystem {
    type SystemData = (Write<'a, GameIsQuit>,);

    fn run(&mut self, data: Self::SystemData) {
        let (mut is_quit,) = data;
        is_quit.0 = true;
    }
}
