use super::*;

use super::dialogue_helpers::{launch_dialogue, DialogueBuilder};

use components::*;
use resources::*;

pub struct HackCallbackHandlerSystem;

#[derive(SystemData)]
pub struct HackCallbackHandlerSystemData<'a> {
    queued_actions: Write<'a, QueuedPlayerActions>,
    hackable: WriteStorage<'a, Hackable>,
    callbacks: Write<'a, Callbacks>,
}

impl<'a> System<'a> for HackCallbackHandlerSystem {
    type SystemData = HackCallbackHandlerSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        let hack_callbacks = data.callbacks.take_some(|cb| match cb {
            Callback::Hack(hdc) => TakeDecision::Take(hdc),
            x => TakeDecision::Leave(x),
        });

        for hcb in hack_callbacks {
            handle_hack_callback(hcb, &mut data.queued_actions, &mut data.hackable, &mut data.callbacks);
        }
    }
}

fn handle_hack_callback(
    hack_callback: HackCallback,
    queued_actions: &mut QueuedPlayerActions,
    hackable: &mut WriteStorage<'_, Hackable>,
    callbacks: &mut Callbacks,
) {
    match hack_callback {
        HackCallback::InitiateHack { target, turn_duration } => {
            for _ in 0..turn_duration {
                queued_actions.action_queue.push_back(QueuedPlayerAction::Wait);
            }

            queued_actions.action_queue.push_back(QueuedPlayerAction::Hack { target });
        }
        HackCallback::ChooseHackTarget { entity } => {
            let hackable = hackable
                .get_mut(entity)
                .expect("If we initiated hack on an entity, it better be hackable");

            let mut builder = DialogueBuilder::new(&format!("Hacking {}...", hackable.name));

            match &hackable.hack_state {
                HackState::Uncompromised => {
                    builder = builder.with_option(
                        "[Compromise]",
                        vec![
                            Callback::Hack(HackCallback::InitiateHack {
                                target: HackTarget {
                                    entity,
                                    hack_type: HackType::Compromise,
                                },
                                turn_duration: 60,
                            }),
                            Callback::EndDialogue,
                        ],
                    );
                }
                HackState::Compromised => {
                    builder = builder
                        .with_option(
                            "[Lock Shut]",
                            vec![
                                Callback::Hack(HackCallback::InitiateHack {
                                    target: HackTarget {
                                        entity,
                                        hack_type: HackType::Door {
                                            new_door_behavior: DoorBehavior::StayClosed,
                                        },
                                    },
                                    turn_duration: 5,
                                }),
                                Callback::EndDialogue,
                            ],
                        )
                        .with_option(
                            "[Lock Open]",
                            vec![
                                Callback::Hack(HackCallback::InitiateHack {
                                    target: HackTarget {
                                        entity,
                                        hack_type: HackType::Door {
                                            new_door_behavior: DoorBehavior::StayOpen,
                                        },
                                    },
                                    turn_duration: 5,
                                }),
                                Callback::EndDialogue,
                            ],
                        )
                        .with_option(
                            "[Set to Automatic]",
                            vec![
                                Callback::Hack(HackCallback::InitiateHack {
                                    target: HackTarget {
                                        entity,
                                        hack_type: HackType::Door {
                                            new_door_behavior: DoorBehavior::FullAuto,
                                        },
                                    },
                                    turn_duration: 5,
                                }),
                                Callback::EndDialogue,
                            ],
                        );
                }
            };

            builder = builder.with_option("[Cancel]", vec![Callback::EndDialogue]);

            launch_dialogue(builder, callbacks);
        }
    }
}
