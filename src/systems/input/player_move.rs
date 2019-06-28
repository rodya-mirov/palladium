use super::*;

use quicksilver::input::{ButtonState, Key, Keyboard};
use specs::Join;

use components::{BlocksMovement, Camera, HasPosition, Player};
use resources::{KeyboardFocus, NpcMoves};

use world::TilePos;

#[derive(SystemData)]
pub struct PlayerMoveSystemData<'a> {
    player: ReadStorage<'a, Player>,
    has_position: WriteStorage<'a, HasPosition>,
    blocks_movement: ReadStorage<'a, BlocksMovement>,
    camera: ReadStorage<'a, Camera>,
    keyboard: ReadExpect<'a, Keyboard>,
    keyboard_focus: Read<'a, KeyboardFocus>,
    npc_moves: Write<'a, NpcMoves>,
}

pub struct PlayerMoveSystem;

impl<'a> System<'a> for PlayerMoveSystem {
    type SystemData = PlayerMoveSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        if *data.keyboard_focus != KeyboardFocus::GameMap {
            return;
        }
        if !data.npc_moves.player_can_move() {
            return;
        }

        let player_move = {
            if data.keyboard[Key::Left] == ButtonState::Pressed {
                Some(TilePos { x: -1, y: 0 })
            } else if data.keyboard[Key::Right] == ButtonState::Pressed {
                Some(TilePos { x: 1, y: 0 })
            } else if data.keyboard[Key::Up] == ButtonState::Pressed {
                Some(TilePos { x: 0, y: -1 })
            } else if data.keyboard[Key::Down] == ButtonState::Pressed {
                Some(TilePos { x: 0, y: 1 })
            } else if data.keyboard[Key::Space] == ButtonState::Pressed {
                Some(TilePos { x: 0, y: 0 })
            } else {
                None
            }
        };

        let mut player_moved = false;

        if let Some(player_move) = player_move {
            let player_pos = get_pos(&data.player, &data.has_position);
            let next_pos = player_pos + player_move;
            if !blocks(next_pos, &data.has_position, &data.blocks_movement) {
                *get_pos_mut(&data.player, &mut data.has_position) += player_move;
                *get_pos_mut(&data.camera, &mut data.has_position) += player_move;
                player_moved = true;
            }
        }

        if player_moved {
            turn_state_helpers::yield_to_npc(&mut data.npc_moves);
        }
    }
}

fn blocks<'a>(next_pos: TilePos, has_pos: &WriteStorage<'a, HasPosition>, blocks: &ReadStorage<'a, BlocksMovement>) -> bool {
    (has_pos, blocks).join().any(|(hp, _)| hp.position == next_pos)
}

fn get_pos<'a, T: Component>(single_comp: &ReadStorage<'a, T>, has_pos: &WriteStorage<'a, HasPosition>) -> TilePos {
    (single_comp, has_pos)
        .join()
        .map(|(_, has_pos)| has_pos)
        .next()
        .expect("Component must be defined and have a position")
        .position
}

fn get_pos_mut<'b, 'a: 'b, T: Component>(
    single_comp: &'b ReadStorage<'a, T>,
    has_pos: &'b mut WriteStorage<'a, HasPosition>,
) -> &'b mut TilePos {
    let has_pos = (single_comp, has_pos)
        .join()
        .map(|(_, has_pos)| has_pos)
        .next()
        .expect("Component must be defined and have a position");

    &mut has_pos.position
}
