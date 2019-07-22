//! Render system for the game world, for rendering "all things which have a position and a glyph"

use super::*;

use std::collections::HashMap;

use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, Image},
};

use specs::Join;

use components::{Camera, CharRender, HasPosition, Visible};
use resources::GameMapRenderParams;

use world::{TilePos, VisibilityType};

use camera_helpers::{get_camera_bounds, CameraBounds};

#[derive(SystemData)]
pub struct CharsRendererSystemData<'a> {
    has_position: ReadStorage<'a, HasPosition>,
    camera: ReadStorage<'a, Camera>,
    visible: ReadStorage<'a, Visible>,
    char_render: ReadStorage<'a, CharRender>,
    game_map_render_params: Read<'a, GameMapRenderParams>,
}

pub struct CharsRendererSetup;

impl<'a> System<'a> for CharsRendererSetup {
    type SystemData = CharsRendererSystemData<'a>;

    fn run(&mut self, _data: Self::SystemData) {}
}

/// System which renders all the "represented by a character" entities
pub struct CharsRenderer<'a> {
    pub window: &'a mut Window,
    pub tileset: &'a mut Asset<HashMap<char, Image>>,
}

struct Renderable<'t> {
    visible: &'t Visible,
    char_render: &'t CharRender,
}

impl<'a, 'b> System<'a> for CharsRenderer<'b> {
    type SystemData = CharsRendererSystemData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let (tileset, window) = (&mut self.tileset, &mut self.window);
        tileset
            .execute(|tileset| {
                let camera_bounds = (&data.has_position, &data.camera)
                    .join()
                    .map(|(&pos, &camera)| get_camera_bounds(pos, camera))
                    .next()
                    .expect("Camera should be defined");

                let mut to_draw: HashMap<TilePos, Vec<Renderable>> = HashMap::new();

                // First, collect all the relevant things
                for (pos, visible, char_render) in (&data.has_position, &data.visible, &data.char_render).join() {
                    if camera_bounds.contains_pos(pos.position) {
                        to_draw
                            .entry(pos.position)
                            .or_insert_with(|| Vec::with_capacity(2))
                            .push(Renderable { visible, char_render });
                    }
                }

                // Then, paint them
                for x in camera_bounds.x_min..=camera_bounds.x_max {
                    for y in camera_bounds.y_min..=camera_bounds.y_max {
                        let draw_renderable = |renderable: Renderable, window: &mut Window| {
                            draw_drawable(
                                renderable.visible.visibility,
                                renderable.char_render,
                                TilePos { x, y },
                                camera_bounds,
                                &data.game_map_render_params,
                                tileset,
                                window,
                            );
                        };

                        let mut draw_here = to_draw.remove(&TilePos { x, y }).unwrap_or_else(Vec::new);

                        draw_here.sort_by_key(|r| r.char_render.z_level);

                        for renderable in draw_here {
                            draw_renderable(renderable, window);
                        }
                    }
                }

                Ok(())
            })
            .expect("Should work!");
    }
}

fn draw_drawable(
    vis: VisibilityType,
    glyph_comp: &CharRender,
    position: TilePos,
    camera_bounds: CameraBounds,
    render_params: &GameMapRenderParams,
    tileset: &HashMap<char, Image>,
    window: &mut Window,
) {
    if glyph_comp.disabled {
        return;
    }

    let render_pos = get_render_pos(position.x, position.y, camera_bounds, *render_params);
    let image = tileset
        .get(&glyph_comp.glyph)
        .unwrap_or_else(|| panic!("Glyph {} should be defined in the tileset", glyph_comp.glyph));

    let rect = Rectangle::new(render_pos, image.area().size());

    window.draw(&rect, Col(with_vis(glyph_comp.bg_color, vis)));
    window.draw(&rect, Blended(&image, with_vis(glyph_comp.fg_color, vis)));
}

fn get_render_pos(x: i32, y: i32, bounds: CameraBounds, render_params: GameMapRenderParams) -> Vector {
    Vector::new(x - bounds.x_min, y - bounds.y_min).times(render_params.tile_size_px) + render_params.map_offset
}

fn with_vis(mut color: Color, visibility: VisibilityType) -> Color {
    let alpha = match visibility {
        VisibilityType::NotSeen => 0.0,
        VisibilityType::Remembered => 0.3,
        VisibilityType::CurrentlyVisible => 1.0,
    };

    color.a *= alpha;
    color
}
