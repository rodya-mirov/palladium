//! A special system, run at game start

use super::*;

pub struct StartGameSystem;

use dialogue_helpers::{launch_dialogue, DialogueBuilder};

use resources::{Callback, Callbacks};

#[derive(SystemData)]
pub struct StartGameSystemData<'a> {
    callbacks: Write<'a, Callbacks>,
}

impl<'a> System<'a> for StartGameSystem {
    type SystemData = StartGameSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        launch_start_dialogue(&mut data.callbacks);
    }
}

fn launch_start_dialogue(callbacks: &mut Callbacks) {
    let text = "Without looking, you know what you will see.\n\nIn front of you, a huge machine, silent, already dead. Metal walls, cold, all around. Beyond that, the vast emptiness of space.\n\nYou have been here before.";

    let builder = DialogueBuilder::new(text).with_option("[Open your eyes]", vec![Callback::EndDialogue, Callback::SaveGame]);

    launch_dialogue(builder, callbacks);
}
