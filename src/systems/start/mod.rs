//! A special system, run at game start

use super::*;

pub struct StartGameSystem;

use dialogue_helpers::{launch_dialogue, DialogueBuilder};

use resources::{DialogueCallback, DialogueStateResource, KeyboardFocus};

#[derive(SystemData)]
pub struct StartGameSystemData<'a> {
    keyboard_focus: Write<'a, KeyboardFocus>,
    dialogue_state_resource: Write<'a, DialogueStateResource>,
}

impl<'a> System<'a> for StartGameSystem {
    type SystemData = StartGameSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        launch_start_dialogue(&mut data.keyboard_focus, &mut data.dialogue_state_resource);
    }
}

fn launch_start_dialogue(focus: &mut KeyboardFocus, dsr: &mut DialogueStateResource) {
    let text = "Without looking, you know what you will see.\n\nIn front of you, a huge machine, silent, already dead. Metal walls, cold, all around. Beyond that, the vast emptiness of space.\n\nYou have been here before.";

    let builder =
        DialogueBuilder::new(text).with_option("[Open your eyes]", vec![DialogueCallback::EndDialogue, DialogueCallback::SaveGame]);

    launch_dialogue(builder, focus, dsr);
}
