use super::*;

use components::{Camera, HasPosition};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct CameraBounds {
    pub x_min: i32,
    pub y_min: i32,
    pub x_max: i32,
    pub y_max: i32,
}

impl CameraBounds {
    pub fn contains_pos(&self, pos: TilePos) -> bool {
        self.x_min <= pos.x && pos.x <= self.x_max && self.y_min <= pos.y && pos.y <= self.y_max
    }
}

pub fn get_camera_bounds(pos: HasPosition, cam: Camera) -> CameraBounds {
    CameraBounds {
        x_min: pos.position.x - cam.x_rad as i32,
        y_min: pos.position.y - cam.y_rad as i32,
        x_max: pos.position.x + cam.x_rad as i32,
        y_max: pos.position.y + cam.y_rad as i32,
    }
}
