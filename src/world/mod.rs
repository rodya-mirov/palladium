//! Module for the "world" aspect of the game state.
//! That is, this tracks all the state of "things in the world" (the player,
//! the camera, the map, other objects, NPCs, ...) as opposed to details of
//! the game engine (stacks of panels, etc.)

use std::ops::Add;

use crate::maps::{Map, SquareType};
use crate::visibility::refresh_visibility;

pub struct World {
    pub map: Map,

    pub player: Player,
    pub camera: CameraInfo,
}

impl World {
    pub fn new(mut map: Map) -> World {
        let camera = CameraInfo {
            x_min: 0,
            x_max: 30,

            y_min: 0,
            y_max: 30,
        };

        // TODO: make this square guaranteed to not be empty
        let player_pos = TilePos { x: 15, y: 15 };

        let player = Player { tile_pos: player_pos };

        refresh_visibility(player_pos, &mut map, 1000); // TODO: vis range

        World { map, camera, player }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct CameraInfo {
    pub x_min: i32,
    pub x_max: i32,

    pub y_min: i32,
    pub y_max: i32,
}

impl CameraInfo {
    pub fn translate(&mut self, translation: TilePos) {
        self.x_min += translation.x;
        self.x_max += translation.x;
        self.y_min += translation.y;
        self.y_max += translation.y;
    }
}

pub struct Player {
    pub tile_pos: TilePos,
}

impl Player {
    pub fn try_move(&mut self, translation: TilePos, map: &Map) -> bool {
        let new_pos = TilePos {
            x: self.tile_pos.x + translation.x,
            y: self.tile_pos.y + translation.y,
        };
        let to_move_to = map.get_square(new_pos.x, new_pos.y);

        let can_move = match to_move_to.square_type {
            SquareType::Void => false,
            SquareType::Wall => false,
            SquareType::Open => false,
            SquareType::Rubbish => false,
            SquareType::Pillar => false,

            SquareType::Floor => true,
            SquareType::Door => true,
        };

        if can_move {
            self.tile_pos = new_pos;
        }

        can_move
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct TilePos {
    pub x: i32,
    pub y: i32,
}

impl Add<TilePos> for TilePos {
    type Output = TilePos;

    fn add(self, rhs: TilePos) -> TilePos {
        TilePos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
