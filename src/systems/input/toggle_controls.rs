use super::*;

use panel::GameMapDisplayOptions;

pub struct ToggleControlSystem;

impl<'a> System<'a> for ToggleControlSystem {
    type SystemData = (Write<'a, GameMapDisplayOptions>, ReadExpect<'a, Keyboard>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut display_options, keyboard) = data;

        if keyboard[Key::C] == ButtonState::Pressed {
            display_options.display_controls_pane = !display_options.display_controls_pane;
        }
    }
}
