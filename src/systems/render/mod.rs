//! Module for systems for rendering.
//!
//! Note that many systems in this module cannot be attached to a dispatcher for
//! lifetime reasons; they must be run manually. Such systems are given a fake/noop
//! system to run, which have the same dependencies and therefore can be used for
//! setting up the resources required.

use super::*;

use quicksilver::{graphics::Image, lifecycle::Window};

use numerics::force_max;

mod image_render_helper;

mod centered_image_renderer;
mod chars_renderer;
mod controls_renderer;
mod oxygen_overlay;

pub use centered_image_renderer::CenteredVerticalImagesRenderer;
pub use chars_renderer::{CharsRenderer, CharsRendererSetup};
pub use controls_renderer::{ControlsRenderer, ControlsRendererSetup};
pub use oxygen_overlay::{OxygenOverlayRenderer, OxygenOverlaySetup};
