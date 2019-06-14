//! The "base" panel, the main game window
//! Renders the map, moves the character, etc.

use quicksilver::input::{ButtonState, Key, Keyboard};
use quicksilver::lifecycle::Window;

use crate::state::{Game, TilePos};
use crate::ui::render_game;
use crate::visibility::refresh_visibility;
use crate::QsResult;

use super::Panel;

pub struct GamePanel {
    // snip?
    is_dead: bool,
}

impl GamePanel {
    pub fn new() -> GamePanel {
        GamePanel { is_dead: false }
    }
}

impl Panel for GamePanel {
    fn update_self(&mut self, _game: &mut Game, _is_active: bool) -> QsResult<()> {
        Ok(())
    }

    fn render_self(&mut self, game: &mut Game, window: &mut Window) -> QsResult<()> {
        render_game(game, window)
    }

    fn do_key_input(&mut self, game: &mut Game, keyboard: &Keyboard) -> QsResult<()> {
        let map = &mut game.map;
        let player = &mut game.player;
        let camera = &mut game.camera;

        if keyboard[Key::C] == ButtonState::Pressed {
            game.controls_pane.show_controls_image = !game.controls_pane.show_controls_image;
        }

        if keyboard[Key::Q] == ButtonState::Pressed {
            self.is_dead = true;
            return Ok(());
        }

        map.execute(|map| {
            let player_move = if keyboard[Key::Left] == ButtonState::Pressed {
                Some(TilePos { x: -1, y: 0 })
            } else if keyboard[Key::Up] == ButtonState::Pressed {
                Some(TilePos { x: 0, y: -1 })
            } else if keyboard[Key::Down] == ButtonState::Pressed {
                Some(TilePos { x: 0, y: 1 })
            } else if keyboard[Key::Right] == ButtonState::Pressed {
                Some(TilePos { x: 1, y: 0 })
            } else {
                None
            };

            if let Some(player_move) = player_move {
                let can_move = player.try_move(player_move, map);
                if can_move {
                    camera.translate(player_move);
                    refresh_visibility(player.tile_pos, map, 1000);
                }
            }

            Ok(())
        })?;

        Ok(())
    }

    fn is_dead(&self) -> bool {
        self.is_dead
    }
}
