use super::*;

use quicksilver::input::{ButtonState, Key, Keyboard};
use specs::Join;

use components::{BlocksMovement, Camera, HasPosition, MapTile, Player};
use world::{TilePos, WorldState};

pub struct PlayerMoveSystem;

impl<'a> System<'a> for PlayerMoveSystem {
    #[allow(clippy::type_complexity)] // more or less inevitable with specs, this is just how it works
    type SystemData = (
        ReadStorage<'a, Player>,
        WriteStorage<'a, HasPosition>,
        ReadStorage<'a, BlocksMovement>,
        ReadStorage<'a, Camera>,
        ReadStorage<'a, MapTile>,
        ReadExpect<'a, Keyboard>,
        ReadExpect<'a, WorldState>,
        Read<'a, KeyboardFocus>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player, mut has_pos, blocks, camera, map_tiles, keyboard, world_state, focus) = data;

        if *focus != KeyboardFocus::GameMap {
            return;
        }

        let player_move = {
            if keyboard[Key::Left] == ButtonState::Pressed {
                Some(TilePos { x: -1, y: 0 })
            } else if keyboard[Key::Right] == ButtonState::Pressed {
                Some(TilePos { x: 1, y: 0 })
            } else if keyboard[Key::Up] == ButtonState::Pressed {
                Some(TilePos { x: 0, y: -1 })
            } else if keyboard[Key::Down] == ButtonState::Pressed {
                Some(TilePos { x: 0, y: 1 })
            } else {
                None
            }
        };

        if let Some(player_move) = player_move {
            let player_pos = get_pos(&player, &has_pos);
            let next_pos = player_pos + player_move;
            if !tile_blocks(next_pos, &blocks, &world_state) && !non_tile_blocks(next_pos, &has_pos, &blocks, &map_tiles) {
                *get_pos_mut(&player, &mut has_pos) += player_move;
                *get_pos_mut(&camera, &mut has_pos) += player_move;
            }
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
