use super::*;

use quicksilver::input::{ButtonState, Key, Keyboard};
use specs::Join;

use components::{BlocksMovement, Camera, HasPosition, MapTile, Player};
use resources::{KeyboardFocus, PlayerHasMoved};

use world::{TilePos, WorldState};

#[derive(SystemData)]
pub struct PlayerMoveSystemData<'a> {
    player: ReadStorage<'a, Player>,
    has_position: WriteStorage<'a, HasPosition>,
    blocks_movement: ReadStorage<'a, BlocksMovement>,
    camera: ReadStorage<'a, Camera>,
    map_tile: ReadStorage<'a, MapTile>,
    keyboard: ReadExpect<'a, Keyboard>,
    world_state: ReadExpect<'a, WorldState>,
    keyboard_focus: Read<'a, KeyboardFocus>,
    player_has_moved: Write<'a, PlayerHasMoved>,
}

pub struct PlayerMoveSystem;

impl<'a> System<'a> for PlayerMoveSystem {
    type SystemData = PlayerMoveSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        if *data.keyboard_focus != KeyboardFocus::GameMap {
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
            if !tile_blocks(next_pos, &data.blocks_movement, &data.world_state)
                && !non_tile_blocks(next_pos, &data.has_position, &data.blocks_movement, &data.map_tile)
            {
                *get_pos_mut(&data.player, &mut data.has_position) += player_move;
                *get_pos_mut(&data.camera, &mut data.has_position) += player_move;
                player_moved = true;
            }
        }

        if player_moved {
            data.player_has_moved.player_has_moved = true;
        }
    }
}

fn tile_blocks<'a>(next_pos: TilePos, blocks: &ReadStorage<'a, BlocksMovement>, world_state: &WorldState) -> bool {
    if let Some(next_tile) = world_state.map.get_tile(next_pos) {
        blocks.get(next_tile).is_some()
    } else {
        // off the map is considered blocking, I guess
        true
    }
}

fn non_tile_blocks<'a>(
    next_pos: TilePos,
    has_pos: &WriteStorage<'a, HasPosition>,
    blocks: &ReadStorage<'a, BlocksMovement>,
    map_tiles: &ReadStorage<'a, MapTile>,
) -> bool {
    (!map_tiles, blocks, has_pos).join().any(|(_, _, pos)| pos.position == next_pos)
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
