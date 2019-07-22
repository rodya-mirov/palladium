//! System which receives player input, only when the "game map" is the active panel

use super::*;

use dialogue_helpers::{launch_dialogue, DialogueBuilder};

use resources::{Callback, Callbacks, GameMapDisplayOptions, KeyboardFocus};

#[derive(SystemData)]
pub struct ToggleControlSystemData<'a> {
    game_map_display_options: Write<'a, GameMapDisplayOptions>,
    keyboard: ReadExpect<'a, Keyboard>,
    keyboard_focus: Write<'a, KeyboardFocus>,

    callbacks: Write<'a, Callbacks>,
}

fn button_down(kb: &Keyboard, key: Key) -> bool {
    match kb[key] {
        ButtonState::Held => true,
        ButtonState::Pressed => true,
        ButtonState::Released => false,
        ButtonState::NotPressed => false,
    }
}

fn shift_held(kb: &Keyboard) -> bool {
    button_down(kb, Key::LShift) || button_down(kb, Key::RShift)
}

pub struct ToggleControlSystem;

impl<'a> System<'a> for ToggleControlSystem {
    type SystemData = ToggleControlSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        if *data.keyboard_focus != KeyboardFocus::GameMap {
            return;
        }

        let keyboard = &mut data.keyboard;
        let display_options = &mut data.game_map_display_options;
        let callbacks = &mut data.callbacks;

        if keyboard[Key::C] == ButtonState::Pressed {
            display_options.display_controls_pane = !display_options.display_controls_pane;
        } else if keyboard[Key::O] == ButtonState::Pressed {
            display_options.show_oxygen_overlay = !display_options.show_oxygen_overlay;
        } else if keyboard[Key::Q] == ButtonState::Pressed {
            launch_quit_dialogue(callbacks);
        } else if keyboard[Key::S] == ButtonState::Pressed {
            data.callbacks.push(Callback::SaveGame);
        } else if keyboard[Key::L] == ButtonState::Pressed {
            if shift_held(&keyboard) {
                launch_license_dialogue(callbacks);
            } else {
                data.callbacks.push(Callback::LoadGame);
            }
        }
    }
}

fn launch_quit_dialogue(callbacks: &mut Callbacks) {
    let builder = DialogueBuilder::new("Quit the game?\nYour progress will not be saved!")
        .with_option("[Cancel]", vec![Callback::EndDialogue])
        .with_option("[Quit]", vec![Callback::EndDialogue, Callback::QuitGame]);

    launch_dialogue(builder, callbacks);
}

fn launch_license_dialogue(callbacks: &mut Callbacks) {
    let text = [
        "Mononoki font by Matthias Tellen, terms: Open Font License 1.1",
        "Square font by Wouter Van Oortmerssen, terms: CC BY 3.0",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect::<Vec<String>>()
    .join("\n");

    let builder = DialogueBuilder::new(&text).with_option("[Continue]", vec![Callback::EndDialogue]);
    launch_dialogue(builder, callbacks);
}
