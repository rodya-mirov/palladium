//! Module for systems for rendering.
//!
//! Note that many systems in this module cannot be attached to a dispatcher for
//! lifetime reasons; they must be run manually. Such systems are given a fake/noop
//! system to run, which have the same dependencies and therefore can be used for
//! setting up the resources required.

use super::*;

use quicksilver::{geom::Shape, graphics::Image, lifecycle::Window, prelude::*};

mod centered_image_renderer;
mod chars_renderer;
mod controls_renderer;

pub use centered_image_renderer::CenteredVerticalImagesRenderer;
pub use chars_renderer::{CharsRenderer, CharsRendererSetup};
pub use controls_renderer::{ControlsRenderer, ControlsRendererSetup};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[allow(dead_code)] // we want to support all these corners for the future
enum Corner {
    UpperLeft,
    UpperRight,
    LowerLeft,
    LowerRight,
}

fn render_image_corner(window: &mut Window, image: &mut Image, offset: Vector, corner: Corner) {
    let screen_size = window.screen_size();
    let image_size = image.area().size();

    let render_pos = get_render_pos_corner(screen_size, image_size, offset, corner);

    window.draw(&image.area().translate(render_pos), Img(&image));
}

fn get_render_pos_corner(screen_size: Vector, image_size: Vector, offset: Vector, corner: Corner) -> Vector {
    match corner {
        Corner::UpperLeft => offset,
        Corner::UpperRight => Vector::new(screen_size.x - image_size.x - offset.x, offset.y),
        Corner::LowerLeft => Vector::new(offset.x, screen_size.y - image_size.y - offset.y),
        Corner::LowerRight => Vector::new(screen_size.x - image_size.x - offset.x, screen_size.y - image_size.y - offset.y),
    }
}

fn force_max(a: f32, b: f32) -> f32 {
    if a.is_nan() {
        b
    } else if b.is_nan() {
        a
    } else if a > b {
        a
    } else {
        b
    }
}
