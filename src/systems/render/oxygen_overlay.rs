//! Render system for the game world, for rendering "all things which have a position and a glyph"

use super::*;

use std::collections::HashMap;

use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::Color,
};

use specs::Join;

use components::{Camera, HasPosition, OxygenContainer, Visible};
use resources::{GameMapDisplayOptions, GameMapRenderParams};

use world::{TilePos, VisibilityType};

#[derive(SystemData)]
pub struct OxygenOverlaySystemData<'a> {
    has_pos: ReadStorage<'a, HasPosition>,
    oxygen_cont: ReadStorage<'a, OxygenContainer>,
    camera: ReadStorage<'a, Camera>,
    visible: ReadStorage<'a, Visible>,
    entities: Entities<'a>,

    render_params: Read<'a, GameMapRenderParams>,
    display_options: Read<'a, GameMapDisplayOptions>,
}

pub struct OxygenOverlaySetup;

impl<'a> System<'a> for OxygenOverlaySetup {
    type SystemData = OxygenOverlaySystemData<'a>;

    fn run(&mut self, _data: Self::SystemData) {
        // no op
    }
}

pub struct OxygenOverlayRenderer<'a> {
    pub window: &'a mut Window,
}

impl<'a, 'b> System<'a> for OxygenOverlayRenderer<'b> {
    type SystemData = OxygenOverlaySystemData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        if !data.display_options.show_oxygen_overlay {
            return;
        }

        let camera_bounds = (&data.has_pos, &data.camera)
            .join()
            .map(|(&pos, &camera)| get_camera_bounds(pos, camera))
            .next()
            .expect("Camera should be defined and have a position");

        let mut oxygen_contents = HashMap::new();

        for (has_pos, ent, vis) in (&data.has_pos, &data.entities, &data.visible).join() {
            // Note: if there are two oxygen containers in a square it will look weird :shrug:
            if camera_bounds.contains_pos(has_pos.position) && vis.visibility == VisibilityType::CurrentlyVisible {
                let local_contents = data.oxygen_cont.get(ent).map(|ox| ox.contents).unwrap_or(0);
                *oxygen_contents.entry(has_pos.position).or_insert(0) += local_contents;
            }
        }

        // First, paint all the relevant tiles (gotten direct from position loop)
        for x in camera_bounds.x_min..=camera_bounds.x_max {
            for y in camera_bounds.y_min..=camera_bounds.y_max {
                // NB: this is None when there is nothing visible in the area
                if let Some(contents) = oxygen_contents.get(&TilePos { x, y }) {
                    let render_pos = get_render_pos(x, y, camera_bounds, *data.render_params);
                    let rect = Rectangle::new(
                        render_pos,
                        Vector::new(data.render_params.font_width, data.render_params.font_width),
                    );
                    self.window.draw(&rect, Col(to_color(*contents)));
                }
            }
        }
    }
}

fn to_color(oxygen_content: usize) -> Color {
    let a = {
        if oxygen_content <= 10 {
            0.5
        } else if oxygen_content <= 20 {
            0.4
        } else if oxygen_content <= 30 {
            0.3
        } else if oxygen_content <= 40 {
            0.2
        } else if oxygen_content <= 50 {
            0.1
        } else {
            0.0
        }
    };

    Color { r: 1.0, g: 0.0, b: 0.0, a }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct CameraBounds {
    x_min: i32,
    y_min: i32,
    x_max: i32,
    y_max: i32,
}

impl CameraBounds {
    fn contains_pos(&self, pos: TilePos) -> bool {
        self.x_min <= pos.x && pos.x <= self.x_max && self.y_min <= pos.y && pos.y <= self.y_max
    }
}

fn get_camera_bounds(pos: HasPosition, cam: Camera) -> CameraBounds {
    CameraBounds {
        x_min: pos.position.x - cam.x_rad as i32,
        y_min: pos.position.y - cam.y_rad as i32,
        x_max: pos.position.x + cam.x_rad as i32,
        y_max: pos.position.y + cam.y_rad as i32,
    }
}

fn get_render_pos(x: i32, y: i32, bounds: CameraBounds, render_params: GameMapRenderParams) -> Vector {
    Vector::new(x - bounds.x_min, y - bounds.y_min).times(render_params.tile_size_px) + render_params.map_offset
}
