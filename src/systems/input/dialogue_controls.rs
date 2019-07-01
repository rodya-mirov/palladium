//! System which handles player input while within a dialogue box

use super::*;

use dialogue_helpers::{end_dialogue, launch_dialogue, DialogueBuilder};

use components::*;
use resources::*;

pub struct DialogueControlSystem;

#[derive(SystemData)]
pub struct DialogueControlSystemData<'a> {
    hackable: WriteStorage<'a, Hackable>,

    keyboard: ReadExpect<'a, Keyboard>,
    game_is_quit: Write<'a, GameIsQuit>,
    keyboard_focus: Write<'a, KeyboardFocus>,
    dialogue_state_resource: Write<'a, DialogueStateResource>,
    queued_actions: Write<'a, QueuedPlayerActions>,
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
            let new_dsr = user_selected(
                get_state(&mut data.dialogue_state_resource),
                &mut data.queued_actions,
                &mut data.hackable,
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

fn user_selected(
    dsr: &mut DialogueState,
    queued_actions: &mut QueuedPlayerActions,
    hackable: &mut WriteStorage<'_, Hackable>,
    focus: &mut KeyboardFocus,
    game_is_quit: &mut GameIsQuit,
) -> DialogueStateResource {
    let mut new_dsr = DialogueStateResource {
        is_initialized: InitializationState::Finished,
        state: Some(dsr.clone()),
    };

    // Note: it's fine to drain these callbacks off, because we're deleting this dsr
    // and replacing it.
    for callback in dsr.options[dsr.selected_index].callbacks.drain(0..) {
        match callback {
            DialogueCallback::EndDialogue => {
                end_dialogue(focus, &mut new_dsr);
            }
            DialogueCallback::Hack(hack_callback) => {
                handle_hack_callback(hack_callback, queued_actions, hackable, focus, &mut new_dsr);
            }
            DialogueCallback::QuitGame => {
                game_is_quit.0 = true;
            }
        }
    }

    new_dsr
}

fn handle_hack_callback(
    hack_callback: HackDialogueCallback,
    queued_actions: &mut QueuedPlayerActions,
    hackable: &mut WriteStorage<'_, Hackable>,
    focus: &mut KeyboardFocus,
    dialogue_state: &mut DialogueStateResource,
) {
    match hack_callback {
        HackDialogueCallback::InitiateHack { target, turn_duration } => {
            for _ in 0..turn_duration {
                queued_actions.action_queue.push_back(QueuedPlayerAction::Wait);
            }

            queued_actions.action_queue.push_back(QueuedPlayerAction::Hack { target });
        }
        HackDialogueCallback::ChooseHackTarget { entity } => {
            let hackable = hackable
                .get_mut(entity)
                .expect("If we initiated hack on an entity, it better be hackable");

            let mut builder = DialogueBuilder::new(&format!("Hacking {}...", hackable.name));

            match &hackable.hack_state {
                HackState::Uncompromised => {
                    builder = builder.with_option(
                        "[Compromise]",
                        vec![
                            DialogueCallback::Hack(HackDialogueCallback::InitiateHack {
                                target: HackTarget {
                                    entity,
                                    hack_type: HackType::Compromise,
                                },
                                turn_duration: 60,
                            }),
                            DialogueCallback::EndDialogue,
                        ],
                    );
                }
                HackState::Compromised => {
                    builder = builder
                        .with_option(
                            "[Lock Shut]",
                            vec![
                                DialogueCallback::Hack(HackDialogueCallback::InitiateHack {
                                    target: HackTarget {
                                        entity,
                                        hack_type: HackType::Door {
                                            new_door_behavior: DoorBehavior::StayClosed,
                                        },
                                    },
                                    turn_duration: 5,
                                }),
                                DialogueCallback::EndDialogue,
                            ],
                        )
                        .with_option(
                            "[Lock Open]",
                            vec![
                                DialogueCallback::Hack(HackDialogueCallback::InitiateHack {
                                    target: HackTarget {
                                        entity,
                                        hack_type: HackType::Door {
                                            new_door_behavior: DoorBehavior::StayOpen,
                                        },
                                    },
                                    turn_duration: 5,
                                }),
                                DialogueCallback::EndDialogue,
                            ],
                        )
                        .with_option(
                            "[Set to Automatic]",
                            vec![
                                DialogueCallback::Hack(HackDialogueCallback::InitiateHack {
                                    target: HackTarget {
                                        entity,
                                        hack_type: HackType::Door {
                                            new_door_behavior: DoorBehavior::FullAuto,
                                        },
                                    },
                                    turn_duration: 5,
                                }),
                                DialogueCallback::EndDialogue,
                            ],
                        );
                }
            };

            builder = builder.with_option("[Cancel]", vec![DialogueCallback::EndDialogue]);

            launch_dialogue(builder, focus, dialogue_state);
        }
    }
}
