use super::*;

use resources::*;

pub struct DialogueUpdateSystem;

impl<'a> System<'a> for DialogueUpdateSystem {
    type SystemData = (Write<'a, Callbacks>, Write<'a, KeyboardFocus>, Write<'a, DialogueStateResource>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut callbacks, mut focus, mut dsr) = data;
        let dialogue_quit_cbs = callbacks.take_some(|cb| match cb {
            Callback::EndDialogue => TakeDecision::Take(()),
            x => TakeDecision::Leave(x),
        });

        if !dialogue_quit_cbs.is_empty() {
            super::dialogue_helpers::end_dialogue(&mut focus, &mut dsr);
        }

        let dialogue_start_cbs = callbacks.take_some(|cb| match cb {
            Callback::StartDialogue(ds) => TakeDecision::Take(ds),
            x => TakeDecision::Leave(x),
        });

        for ds in dialogue_start_cbs {
            if *focus != KeyboardFocus::GameMap {
                panic!("Can only start dialogue from game map!");
            }

            *focus = KeyboardFocus::Dialogue;
            *dsr = DialogueStateResource {
                is_initialized: InitializationState::NotStarted,
                state: Some(ds),
            };
        }
    }
}

pub struct PlayerNotMoved;

impl<'a> System<'a> for PlayerNotMoved {
    type SystemData = (Write<'a, NpcMoves>, Write<'a, GameClock>, Write<'a, RenderStale>);

    fn run(&mut self, mut data: Self::SystemData) {
        turn_state_helpers::timestep(&mut data.0, &mut data.1, &mut data.2);
    }
}

pub struct GameIsQuitCheckerSystem;

impl<'a> System<'a> for GameIsQuitCheckerSystem {
    type SystemData = (Write<'a, Callbacks>, Write<'a, GameIsQuit>);

    fn run(&mut self, mut data: Self::SystemData) {
        let quit_cbs = data.0.take_some(|cb| match cb {
            Callback::QuitGame => TakeDecision::Take(()),
            x => TakeDecision::Leave(x),
        });

        if !quit_cbs.is_empty() {
            (data.1).0 = true;
        }
    }
}

pub struct CallbackCheckerSystem;

impl<'a> System<'a> for CallbackCheckerSystem {
    type SystemData = (Read<'a, Callbacks>);

    fn run(&mut self, data: Self::SystemData) {
        if !data.is_empty() {
            panic!("Callbacks must be handled every timestep. Unhandled callbacks: {:?}", *data);
        }
    }
}
