//! System which handles player input while within a dialogue box

use super::*;

use resources::*;

pub struct DialogueControlSystem;

#[derive(SystemData)]
pub struct DialogueControlSystemData<'a> {
    keyboard: ReadExpect<'a, Keyboard>,
    keyboard_focus: Write<'a, KeyboardFocus>,
    dialogue_state_resource: Write<'a, DialogueStateResource>,
    callbacks: Write<'a, Callbacks>,
}

// NB: took out space because it's too easy to miss dialogues when you're wait spamming
const ACCEPT_KEYS: [Key; 1] = [Key::Return];

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
            let ds = get_state(&mut data.dialogue_state_resource);

            for callback in &ds.options[ds.selected_index].callbacks {
                data.callbacks.push(callback.clone());
            }
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
