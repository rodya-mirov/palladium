//! Render system for the controls pane. Fundamentally just a "render this at the upper right
//! corner of the screen"

use super::*;

use quicksilver::graphics::Image;

use panel::{GameMapDisplayOptions, GameMapRenderParams};

type ControlersRendererData<'a> = (Read<'a, GameMapRenderParams>, Read<'a, GameMapDisplayOptions>);

/// Noop system for setup
pub struct ControlsRendererSetup;

impl<'a> System<'a> for ControlsRendererSetup {
    type SystemData = ControlersRendererData<'a>;

    fn run(&mut self, _data: Self::SystemData) {}
}

/// System which renders all the controls stuff
pub struct ControlsRenderer<'a> {
    pub window: &'a mut Window,
    pub controls_image: &'a mut Asset<Image>,
}

impl<'a, 'b> System<'a> for ControlsRenderer<'b> {
    type SystemData = ControlersRendererData<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let (controls_image, window) = (&mut self.controls_image, &mut self.window);
        let (render_params, display_options) = data;
        if !display_options.display_controls_pane {
            return;
        }

        controls_image
            .execute(|image| {
                let offset = render_params.controls_image_offset_px;

                super::render_image_corner(window, image, offset, Corner::UpperRight);

                Ok(())
            })
            .expect("Should work!");
    }
}
