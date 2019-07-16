//! System which receives player input, only when the "game map" is the active panel

use super::*;

use dialogue_helpers::{launch_dialogue, DialogueBuilder};

use resources::{DialogueCallback, DialogueStateResource, GameMapDisplayOptions, KeyboardFocus, SavedStates};

#[derive(SystemData)]
pub struct ToggleControlSystemData<'a> {
    game_map_display_options: Write<'a, GameMapDisplayOptions>,
    keyboard: ReadExpect<'a, Keyboard>,
    keyboard_focus: Write<'a, KeyboardFocus>,
    saved_states: Write<'a, SavedStates>,
    dialogue_state_resource: Write<'a, DialogueStateResource>,
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
        let focus = &mut data.keyboard_focus;
        let dsr = &mut data.dialogue_state_resource;
        let display_options = &mut data.game_map_display_options;

        if keyboard[Key::C] == ButtonState::Pressed {
            display_options.display_controls_pane = !display_options.display_controls_pane;
        } else if keyboard[Key::O] == ButtonState::Pressed {
            display_options.show_oxygen_overlay = !display_options.show_oxygen_overlay;
        } else if keyboard[Key::Q] == ButtonState::Pressed {
            launch_quit_dialogue(focus, dsr);
        } else if keyboard[Key::S] == ButtonState::Pressed {
            data.saved_states.save_requested = true;
        } else if keyboard[Key::L] == ButtonState::Pressed {
            if shift_held(&keyboard) {
                launch_license_dialogue(focus, dsr);
            } else {
                data.saved_states.load_requested = true;
            }
        }
    }
}

fn launch_quit_dialogue(focus: &mut KeyboardFocus, dsr: &mut DialogueStateResource) {
    let builder = DialogueBuilder::new("Quit the game?\nYour progress will not be saved!")
        .with_option("[Cancel]", vec![DialogueCallback::EndDialogue])
        .with_option("[Quit]", vec![DialogueCallback::EndDialogue, DialogueCallback::QuitGame]);

    launch_dialogue(builder, focus, dsr);
}

fn launch_license_dialogue(focus: &mut KeyboardFocus, dsr: &mut DialogueStateResource) {
    let text = [
        "Mononoki font by Matthias Tellen, terms: Open Font License 1.1",
        "Square font by Wouter Van Oortmerssen, terms: CC BY 3.0",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect::<Vec<String>>()
    .join("\n");

    let builder = DialogueBuilder::new(&text).with_option("[Continue]", vec![DialogueCallback::EndDialogue]);
    launch_dialogue(builder, focus, dsr);
}
