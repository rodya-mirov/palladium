//! System which handles player input while within a dialogue box

use super::*;

use game_state::{DialogueCallback, DialogueState, GameIsQuit, KeyboardFocus};

pub struct DialogueControlSystem;

#[derive(SystemData)]
pub struct DialogueControlSystemData<'a> {
    keyboard: ReadExpect<'a, Keyboard>,
    game_is_quit: Write<'a, GameIsQuit>,
    keyboard_focus: Write<'a, KeyboardFocus>,
    dialogue_state_resource: Write<'a, DialogueStateResource>,
}

const ACCEPT_KEYS: [Key; 2] = [Key::Space, Key::Return];

impl<'a> System<'a> for DialogueControlSystem {
    type SystemData = DialogueControlSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        if *data.keyboard_focus != KeyboardFocus::Dialogue || data.dialogue_state_resource.is_initialized != InitializationState::Finished {
            return;
        }

        if data.keyboard[Key::Up] == ButtonState::Pressed {
            user_up(get_state(&mut data.dialogue_state_resource));
        } else if data.keyboard[Key::Down] == ButtonState::Pressed {
            user_down(get_state(&mut data.dialogue_state_resource));
        } else if ACCEPT_KEYS.iter().any(|key| data.keyboard[*key] == ButtonState::Pressed) {
            let new_dsr = user_selected(
                get_state(&mut data.dialogue_state_resource),
                &mut data.keyboard_focus,
                &mut data.game_is_quit,
            );
            *data.dialogue_state_resource = new_dsr;
        }
    }
}

fn get_state(dsr: &mut DialogueStateResource) -> &mut DialogueState {
    dsr.state.as_mut().expect("If dialogue is open, dialogue state should exist")
}

fn user_up(dsr: &mut DialogueState) {
    if dsr.selected_index > 0 {
        dsr.selected_index -= 1;
    }
}

fn user_down(dsr: &mut DialogueState) {
    if dsr.selected_index < dsr.options.len() - 1 {
        dsr.selected_index += 1;
    }
}

fn user_selected(dsr: &mut DialogueState, focus: &mut KeyboardFocus, game_is_quit: &mut GameIsQuit) -> DialogueStateResource {
    let mut new_dsr = DialogueStateResource {
        is_initialized: InitializationState::Finished,
        state: Some(dsr.clone()),
    };

    for callback in &dsr.options[dsr.selected_index].callbacks {
        match callback {
            DialogueCallback::EndDialogue => {
                end_dialogue(focus, &mut new_dsr);
            }
            DialogueCallback::QuitGame => {
                game_is_quit.0 = true;
            }
        }
    }

    new_dsr
}
