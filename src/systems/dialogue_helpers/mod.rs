use super::*;

use resources::*;

pub struct DialogueBuilder {
    main_text: String,
    selected_index: usize,
    options: Vec<DialogueOptionBuilder>,
}

struct DialogueOptionBuilder {
    selected_text: String,
    unselected_text: String,
    callbacks: Vec<DialogueCallback>,
}

impl DialogueBuilder {
    pub fn new(main_text: &str) -> Self {
        DialogueBuilder {
            main_text: main_text.to_string(),
            selected_index: 0,
            options: Vec::with_capacity(1),
        }
    }

    pub fn with_option(mut self, text: &str, callbacks: Vec<DialogueCallback>) -> Self {
        self.options.push(DialogueOptionBuilder {
            selected_text: text.to_string(),
            unselected_text: text.to_string(),
            callbacks,
        });
        self
    }

    pub fn build(self) -> DialogueState {
        if self.selected_index >= self.options.len() {
            panic!(
                "Cannot initialize this dialogue builder, because the selected index {} is out of range: {}",
                self.selected_index,
                self.options.len()
            );
        }

        DialogueState {
            main_text: self.main_text,
            selected_index: self.selected_index,
            options: self.options.into_iter().map(|opt| opt.build()).collect(),
        }
    }
}

impl DialogueOptionBuilder {
    pub fn build(self) -> DialogueOptionState {
        DialogueOptionState {
            selected_text: self.selected_text,
            unselected_text: self.unselected_text,
            callbacks: self.callbacks,
        }
    }
}

pub fn end_dialogue(focus: &mut KeyboardFocus, dialogue_state: &mut DialogueStateResource) {
    if *focus != KeyboardFocus::Dialogue {
        panic!("Cannot end dialogue when it is not running!");
    }

    *focus = KeyboardFocus::GameMap;
    *dialogue_state = DialogueStateResource {
        is_initialized: InitializationState::NotStarted,
        state: None,
    };
}

pub fn launch_dialogue(builder: DialogueBuilder, focus: &mut KeyboardFocus, dialogue_state: &mut DialogueStateResource) {
    if *focus != KeyboardFocus::GameMap {
        panic!("Can only start dialogue from game map!");
    }

    *focus = KeyboardFocus::Dialogue;
    *dialogue_state = DialogueStateResource {
        is_initialized: InitializationState::NotStarted,
        state: Some(builder.build()),
    };
}
