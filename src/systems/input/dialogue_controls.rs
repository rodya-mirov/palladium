//! System which handles player input while within a dialogue box

use super::*;

use game_state::{DialogueCallback, DialogueState, GameIsQuit, KeyboardFocus};

pub struct DialogueControlSystem;

type DialogueControlSystemData<'a> = (
    ReadExpect<'a, Keyboard>,
    Write<'a, GameIsQuit>,
    Write<'a, KeyboardFocus>,
    Write<'a, DialogueStateResource>,
);

const ACCEPT_KEYS: [Key; 2] = [Key::Space, Key::Return];

impl<'a> System<'a> for DialogueControlSystem {
    type SystemData = DialogueControlSystemData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let (keyboard, mut game_is_quit, mut focus, mut dsr) = data;

        if *focus != KeyboardFocus::Dialogue || dsr.is_initialized != InitializationState::Finished {
            return;
        }

        if keyboard[Key::Up] == ButtonState::Pressed {
            user_up(get_state(&mut dsr));
        } else if keyboard[Key::Down] == ButtonState::Pressed {
            user_down(get_state(&mut dsr));
        } else if ACCEPT_KEYS.iter().any(|key| keyboard[*key] == ButtonState::Pressed) {
            let new_dsr = user_selected(get_state(&mut dsr), &mut focus, &mut game_is_quit);
            *dsr = new_dsr;
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
