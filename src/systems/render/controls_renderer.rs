//! Render system for the controls pane. Fundamentally just a "render this at the upper right
//! corner of the screen"

use super::*;

use std::collections::HashMap;

use quicksilver::graphics::Image;

use resources::{GameClock, GameMapDisplayOptions, GameMapRenderParams};

use image_render_helper::{render_image_corner, render_images_corner, Alignment, Corner};
use numerics::force_max;

#[derive(SystemData)]
pub struct ControlsRendererSystemData<'a> {
    game_clock: Read<'a, GameClock>,
    game_map_render_params: Read<'a, GameMapRenderParams>,
    game_map_display_options: Read<'a, GameMapDisplayOptions>,
}

/// Noop system for setup
pub struct ControlsRendererSetup;

impl<'a> System<'a> for ControlsRendererSetup {
    type SystemData = ControlsRendererSystemData<'a>;

    fn run(&mut self, _data: Self::SystemData) {}
}

/// System which renders all the controls stuff
pub struct ControlsRenderer<'a> {
    pub window: &'a mut Window,
    pub controls_image: &'a mut Asset<Image>,
    pub tileset: &'a mut Asset<HashMap<char, Image>>,
}

impl<'a, 'b> System<'a> for ControlsRenderer<'b> {
    type SystemData = ControlsRendererSystemData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let window = &mut self.window;
        if !data.game_map_display_options.display_controls_pane {
            return;
        }

        self.controls_image
            .execute(|image| {
                let offset = data.game_map_render_params.controls_image_offset_px;

                render_image_corner(window, image, offset, Corner::UpperRight);

                Ok(())
            })
            .expect("Should work!");

        self.tileset
            .execute(|tileset| {
                let time_string = format!(
                    "{:02}:{:02}:{:02}",
                    data.game_clock.hours, data.game_clock.minutes, data.game_clock.seconds
                );

                let mut total_width = 0.0;
                let mut max_height = 0.0;

                let to_render: Vec<&Image> = time_string
                    .chars()
                    .map(|c| {
                        tileset
                            .get(&c)
                            .unwrap_or_else(|| panic!("Should have defined a tileset item for {}", c))
                    })
                    .map(|img| {
                        let area = img.area().size();
                        max_height = force_max(max_height, area.y);
                        total_width += area.x;
                        img
                    })
                    .collect();

                render_images_corner(
                    window,
                    &to_render,
                    data.game_map_render_params.controls_image_offset_px,
                    Vector::new(0, 0),
                    Corner::UpperLeft,
                    Alignment::Horizontal,
                );

                Ok(())
            })
            .expect("Rendering should work");
    }
}
