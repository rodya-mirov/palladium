//! Module for describing "panels", the basic unit of flow control in this "engine"
//! Panels "stack" in the following sense:
//! - For rendering:
//!     - Render is called, bottom to top (so "latter" covers "earlier")
//! - For updating:
//!     - Update is called, bottom to top (so "latter" covers "earlier"), then ...
//!     - do_key_input is called ONLY ON THE TOP, then ...
//!     - From top to bottom, each panel is asked if it is dead; if so, it is removed.
//!         If a non-dead panel is found, the loop stops

use super::*;

use quicksilver::input::Keyboard;
use quicksilver::lifecycle::Window;

use specs::World;

use crate::QsResult;

mod game_panel;
mod menu_panel;

pub use game_panel::{GameIsQuit, GameMapDisplayOptions, GameMapRenderParams, GamePanel};
pub use menu_panel::{make_license_panel, make_quit_panel};

pub type PanelResult = QsResult<Vec<PanelAction>>;

pub enum PanelAction {
    AddPanelAbove(Box<dyn Panel>),
    AddPanelBehind(Box<dyn Panel>),
    CloseCurrentPanel,
}

pub trait Panel {
    fn update_self(&mut self, world: &mut World, is_active: bool) -> PanelResult;
    fn render_self(&mut self, world: &mut World, window: &mut Window) -> PanelResult;
    fn do_key_input(&mut self, world: &mut World, keyboard: &Keyboard) -> PanelResult;
}
