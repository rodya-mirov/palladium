//! Module for describing "panels", the basic unit of flow control in this "engine"
//! Panels "stack" in the following sense:
//! - For rendering:
//!   - Render is called, bottom to top (so "latter" covers "earlier")
//! - For updating:
//!     - Update is called, bottom to top (so "latter" covers "earlier"), then
//!     - do_key_input is called ONLY ON THE TOP
//!     - From top to bottom, each panel is asked if it is dead; if so, it is removed
//!         If a non-dead panel is found, the loop stops

use quicksilver::input::Keyboard;
use quicksilver::lifecycle::Window;

use crate::state::Game;
use crate::QsResult;

mod game_panel;

pub trait Panel {
    fn update_self(&mut self, game: &mut Game, is_active: bool) -> QsResult<()>;
    fn render_self(&mut self, game: &mut Game, window: &mut Window) -> QsResult<()>;
    fn do_key_input(&mut self, game: &mut Game, keyboard: &Keyboard) -> QsResult<()>;
    fn is_dead(&self) -> bool;
}
