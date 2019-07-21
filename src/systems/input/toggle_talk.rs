//! System which receives player input, only when the "game map" is the active panel

use super::*;

use dialogue_helpers::{launch_dialogue, DialogueBuilder};

use components::*;
use resources::*;

#[derive(SystemData)]
pub struct ToggleTalkSystemData<'a> {
    player: ReadStorage<'a, Player>,
    has_position: ReadStorage<'a, HasPosition>,
    talkables: ReadStorage<'a, Talkable>,
    entities: Entities<'a>,

    keyboard: ReadExpect<'a, Keyboard>,
    npc_moves: Read<'a, NpcMoves>,
    keyboard_focus: Read<'a, KeyboardFocus>,
    callbacks: Write<'a, Callbacks>,
}

pub struct ToggleTalkSystem;

impl<'a> System<'a> for ToggleTalkSystem {
    type SystemData = ToggleTalkSystemData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        if *data.keyboard_focus != KeyboardFocus::GameMap {
            return;
        }

        if !data.npc_moves.player_can_move() {
            return;
        }

        if data.keyboard[Key::T] == ButtonState::Pressed {
            launch_talk(data);
        }
    }
}

fn launch_talk(mut data: ToggleTalkSystemData<'_>) {
    let player_pos = (&data.player, &data.has_position)
        .join()
        .map(|(_, has_pos)| has_pos.position)
        .next()
        .expect("There should be a player with a position");

    let neighbor_positions = direct_neighbors(player_pos);

    let talkables: Vec<(&Talkable, Direction, Entity)> = (&data.talkables, &data.has_position, &data.entities)
        .join()
        .filter(|(_, hp, _)| is_neighbor(hp.position, &neighbor_positions))
        .map(|(talkable, hp, entity)| (talkable, get_direction(player_pos, hp.position), entity))
        // TODO: some kind of nice sorting by position?
        .collect();

    if talkables.is_empty() {
        launch_no_talks_dialogue(&mut data.callbacks);
    } else {
        choose_talk_target_dialogue(talkables, &mut data.callbacks);
    }
}

fn is_neighbor(pos: TilePos, neighbor_positions: &[TilePos]) -> bool {
    neighbor_positions.iter().any(|&np| np == pos)
}

fn launch_no_talks_dialogue(callbacks: &mut Callbacks) {
    let builder = DialogueBuilder::new("There is no one nearby to talk to.").with_option("[Continue]", vec![Callback::EndDialogue]);

    launch_dialogue(builder, callbacks);
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

fn talkable_name(talkable: &Talkable, dir: Direction) -> String {
    let dir_string = to_string(dir);

    format!("[{} ({})]", talkable.name, dir_string)
}

fn choose_talk_target_dialogue(talkables: Vec<(&Talkable, Direction, Entity)>, callbacks: &mut Callbacks) {
    let mut builder = DialogueBuilder::new("Who or what do you want to talk to?");

    for talkable in talkables {
        let name = talkable_name(talkable.0, talkable.1);
        let entity = talkable.2;
        builder = builder.with_option(
            &name,
            vec![Callback::EndDialogue, Callback::Talk(TalkCallback::ChooseTalkTarget { entity })],
        );
    }

    builder = builder.with_option("[Cancel]", vec![Callback::EndDialogue]);

    launch_dialogue(builder, callbacks);
}
