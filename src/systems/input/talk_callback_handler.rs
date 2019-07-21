use super::*;

use super::dialogue_helpers::{launch_dialogue, DialogueBuilder};

use components::*;
use resources::*;

pub struct TalkCallbackHandlerSystem;

#[derive(SystemData)]
pub struct TalkCallbackHandlerSystemData<'a> {
    talkable: ReadStorage<'a, Talkable>,
    callbacks: Write<'a, Callbacks>,
}

impl<'a> System<'a> for TalkCallbackHandlerSystem {
    type SystemData = TalkCallbackHandlerSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        let talk_callbacks = data.callbacks.take_some(|cb| match cb {
            Callback::Talk(tcb) => TakeDecision::Take(tcb),
            x => TakeDecision::Leave(x),
        });

        for tcb in talk_callbacks {
            handle_talk_callback(tcb, &mut data.talkable, &mut data.callbacks);
        }
    }
}

fn handle_talk_callback(tcb: TalkCallback, talkable: &mut ReadStorage<'_, Talkable>, callbacks: &mut Callbacks) {
    match tcb {
        TalkCallback::ChooseTalkTarget { entity } => {
            // just getting it to make sure it exists
            let _talkable = talkable
                .get(entity)
                .expect("If we initiated talk on an entity, it better be talkable");

            // TODO: obviously at some point we want a whole sophisticated dialogue tree structure here
            let mut builder = DialogueBuilder::new("The creature faces you and makes sounds and gestures you cannot understand");

            builder = builder
                .with_option("[Try to imitate its language]", vec![])
                .with_option("[Give up]", vec![Callback::EndDialogue]);

            launch_dialogue(builder, callbacks);
        }
    }
}
