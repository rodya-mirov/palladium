//! Render system for the game world, for rendering "all things which have a position and a glyph"

use super::*;

use std::collections::HashMap;

use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, Image},
};

use specs::Join;

use components::{Camera, CharRender, HasPosition, MapTile, Visible};
use panel::GameMapRenderParams;
use world::{TilePos, VisibilityType, WorldState};

type CharsRendererData<'a> = (
    ReadStorage<'a, HasPosition>,
    ReadStorage<'a, MapTile>,
    ReadStorage<'a, Camera>,
    ReadStorage<'a, Visible>,
    ReadStorage<'a, CharRender>,
    ReadExpect<'a, WorldState>,
    Read<'a, GameMapRenderParams>,
);

pub struct CharsRendererSetup;

impl<'a> System<'a> for CharsRendererSetup {
    type SystemData = CharsRendererData<'a>;

    fn run(&mut self, _data: Self::SystemData) {}
}

/// System which renders all the "represented by a character" entities
pub struct CharsRenderer<'a> {
    pub window: &'a mut Window,
    pub tileset: &'a mut Asset<HashMap<char, Image>>,
}

const BG_COLOR: Color = Color::BLUE;

impl<'a, 'b> System<'a> for CharsRenderer<'b> {
    type SystemData = CharsRendererData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let (tileset, window) = (&mut self.tileset, &mut self.window);
        tileset
            .execute(|tileset| {
                let (pos, tile, camera, vis, char_render, world_state, render_params) = data;
                let map = &world_state.map;

                let camera_bounds = (&pos, &camera)
                    .join()
                    .map(|(&pos, &camera)| get_camera_bounds(pos, camera))
                    .next()
                    .expect("Camera should be defined");

                // First, paint all the relevant tiles (gotten direct from position loop)
                for x in camera_bounds.x_min..=camera_bounds.x_max {
                    for y in camera_bounds.y_min..=camera_bounds.y_max {
                        if let Some(tile_entity) = map.get_tile(TilePos { x, y }) {
                            let vis = vis.get(tile_entity).expect("Tiles should have visibility").visibility;
                            let glyph_comp = char_render.get(tile_entity).expect("Tiles should have glyphs");
                            let render_pos = get_render_pos(x, y, camera_bounds, *render_params);
                            let image = tileset
                                .get(&glyph_comp.glyph)
                                .unwrap_or_else(|| panic!("Glyph {} should be defined in the tileset", glyph_comp.glyph));

                            let rect = Rectangle::new(render_pos, image.area().size());

                            window.draw(&rect, with_vis(BG_COLOR, vis));
                            window.draw(&rect, Blended(&image, with_vis(glyph_comp.fg_color, vis)));
                        }
                    }
                }

                // Then, paint all entities with enough attributes, filtered by camera
                for (&pos, _, &vis, &glyph) in (&pos, !&tile, &vis, &char_render)
                    .join()
                    .filter(|(pos, _, _, _)| camera_bounds.contains_pos(pos.position))
                {
                    if let Some(image) = tileset.get(&glyph.glyph) {
                        let render_pos = get_render_pos(pos.position.x, pos.position.y, camera_bounds, *render_params);
                        let rect = Rectangle::new(render_pos, image.area().size());

                        window.draw(&rect, with_vis(BG_COLOR, vis.visibility));
                        window.draw(&rect, Blended(&image, with_vis(glyph.fg_color, vis.visibility)));
                    }
                }

                Ok(())
            })
            .expect("Should work!");
    }
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

fn with_vis(color: Color, visibility: VisibilityType) -> Color {
    let alpha = match visibility {
        VisibilityType::NotSeen => 0.0,
        VisibilityType::Remembered => 0.3,
        VisibilityType::CurrentlyVisible => 1.0,
    };

    color.with_alpha(alpha)
}
