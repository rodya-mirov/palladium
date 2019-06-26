//! System which receives player input, only when the "game map" is the active panel

use super::*;

use components::*;
use game_state::{DialogueCallback, HackDialogueCallback, KeyboardFocus};

#[derive(SystemData)]
pub struct ToggleHackSystemData<'a> {
    player: ReadStorage<'a, Player>,
    has_position: ReadStorage<'a, HasPosition>,
    hackables: ReadStorage<'a, Hackable>,
    entities: Entities<'a>,

    keyboard: ReadExpect<'a, Keyboard>,
    keyboard_focus: Write<'a, KeyboardFocus>,
    dialogue_state_resource: Write<'a, DialogueStateResource>,
}

pub struct ToggleHackSystem;

impl<'a> System<'a> for ToggleHackSystem {
    type SystemData = ToggleHackSystemData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        if *data.keyboard_focus != KeyboardFocus::GameMap {
            return;
        }

        if data.keyboard[Key::H] == ButtonState::Pressed {
            launch_hack(data);
        }
    }
}

fn launch_hack(mut data: ToggleHackSystemData<'_>) {
    let player_pos = (&data.player, &data.has_position)
        .join()
        .map(|(_, has_pos)| has_pos.position)
        .next()
        .expect("There should be a player with a position");

    let neighbor_positions = direct_neighbors(player_pos);

    let hackables: Vec<(&Hackable, Direction, Entity)> = (&data.hackables, &data.has_position, &data.entities)
        .join()
        .filter(|(_, hp, _)| is_neighbor(hp.position, &neighbor_positions))
        .map(|(hackable, hp, entity)| (hackable, get_direction(player_pos, hp.position), entity))
        // TODO: some kind of nice sorting by position?
        .collect();

    if hackables.is_empty() {
        launch_no_hacks_dialogue(&mut data.keyboard_focus, &mut data.dialogue_state_resource);
    } else {
        choose_hack_target_dialogue(hackables, &mut data.keyboard_focus, &mut data.dialogue_state_resource);
    }
}

fn is_neighbor(pos: TilePos, neighbor_positions: &[TilePos]) -> bool {
    neighbor_positions.iter().any(|&np| np == pos)
}

fn launch_no_hacks_dialogue(focus: &mut KeyboardFocus, dsr: &mut DialogueStateResource) {
    let builder =
        DialogueBuilder::new("There are no nearby hackable objects.").with_option("[Continue]", vec![DialogueCallback::EndDialogue]);

    launch_dialogue(builder, focus, dsr);
}

fn get_direction(my_pos: TilePos, other_pos: TilePos) -> Direction {
    if my_pos.y < other_pos.y {
        Direction::South
    } else if my_pos.y > other_pos.y {
        Direction::North
    } else if my_pos.x < other_pos.x {
        Direction::East
    } else if my_pos.x > other_pos.x {
        Direction::West
    } else {
        Direction::Here
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
    Here,
}

fn to_string(dir: Direction) -> &'static str {
    match dir {
        Direction::South => "South",
        Direction::East => "East",
        Direction::North => "North",
        Direction::West => "West",
        Direction::Here => "Here",
    }
}

fn hackable_name(hackable: &Hackable, dir: Direction) -> String {
    let dir_string = to_string(dir);

    match &hackable.hack_state {
        HackState::Door(DoorHackState::Uncompromised) => format!("[{} ({})]", hackable.name, dir_string),
        HackState::Door(DoorHackState::CompromisedNormal) => format!("[{} (compromised) ({})]", hackable.name, dir_string),
        HackState::Door(DoorHackState::CompromisedOpen) => format!("[{} (compromised: locked open) ({})]", hackable.name, dir_string),
        HackState::Door(DoorHackState::CompromisedShut) => format!("[{} (compromised: locked shut) ({})]", hackable.name, dir_string),
    }
}

fn choose_hack_target_dialogue(hackables: Vec<(&Hackable, Direction, Entity)>, focus: &mut KeyboardFocus, dsr: &mut DialogueStateResource) {
    let mut builder = DialogueBuilder::new("Which object to hack?");

    for hackable in hackables {
        let name = hackable_name(hackable.0, hackable.1);
        let entity = hackable.2;
        builder = builder.with_option(
            &name,
            vec![
                DialogueCallback::EndDialogue,
                DialogueCallback::Hack(HackDialogueCallback::ChooseHackTarget(entity)),
            ],
        );
    }

    builder = builder.with_option("[Cancel]", vec![DialogueCallback::EndDialogue]);

    launch_dialogue(builder, focus, dsr);
}
