//! System which receives player input, only when the "game map" is the active panel

use super::*;

use game_state::{DialogueCallback, GameMapDisplayOptions, KeyboardFocus};

pub struct ToggleControlSystem;

impl<'a> System<'a> for ToggleControlSystem {
    type SystemData = (
        Write<'a, GameMapDisplayOptions>,
        ReadExpect<'a, Keyboard>,
        Write<'a, KeyboardFocus>,
        Write<'a, DialogueStateResource>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut display_options, keyboard, mut focus, mut dsr) = data;

        if *focus != KeyboardFocus::GameMap {
            return;
        }

        if keyboard[Key::C] == ButtonState::Pressed {
            display_options.display_controls_pane = !display_options.display_controls_pane;
        } else if keyboard[Key::O] == ButtonState::Pressed {
            display_options.show_oxygen_overlay = !display_options.show_oxygen_overlay;
        } else if keyboard[Key::Q] == ButtonState::Pressed {
            let builder = DialogueBuilder::new("Quit the game?\nYour progress will not be saved!")
                .with_option("[Cancel]", vec![DialogueCallback::EndDialogue])
                .with_option("[Quit]", vec![DialogueCallback::EndDialogue, DialogueCallback::QuitGame]);

            launch_dialogue(builder, &mut focus, &mut dsr);
        } else if keyboard[Key::L] == ButtonState::Pressed {
            let text = [
                "Mononoki font by Matthias Tellen, terms: Open Font License 1.1",
                "Square font by Wouter Van Oortmerssen, terms: CC BY 3.0",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join("\n");

            let builder = DialogueBuilder::new(&text).with_option("[Continue]", vec![DialogueCallback::EndDialogue]);
            launch_dialogue(builder, &mut focus, &mut dsr);
        }
    }
}
