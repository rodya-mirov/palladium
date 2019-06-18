//! The "base" panel, the main game window
//! Renders the map, moves the character, etc.

use quicksilver::input::{ButtonState, Key, Keyboard};
use quicksilver::lifecycle::Window;

use crate::state::Game;
use crate::ui::render_game;
use crate::visibility::refresh_visibility;
use crate::world::TilePos;

use super::{Panel, PanelAction, PanelResult};

pub struct GamePanel;

impl GamePanel {
    pub fn new() -> GamePanel {
        GamePanel
    }
}

impl Panel for GamePanel {
    fn update_self(&mut self, _game: &mut Game, _is_active: bool) -> PanelResult {
        Ok(Vec::new())
    }

    fn render_self(&mut self, game: &mut Game, window: &mut Window) -> PanelResult {
        render_game(game, window)?;
        Ok(Vec::new())
    }

    fn do_key_input(&mut self, game: &mut Game, keyboard: &Keyboard) -> PanelResult {
        if game.is_quit {
            return Ok(vec![PanelAction::CloseCurrentPanel]);
        }
        let mut actions = Vec::new();

        if keyboard[Key::C] == ButtonState::Pressed {
            game.controls_pane.show_controls_image = !game.controls_pane.show_controls_image;
        }

        if keyboard[Key::Q] == ButtonState::Pressed {
            use super::quit_panel::make_quit_panel;
            actions.push(PanelAction::AddPanelAbove(Box::new(make_quit_panel())));
        }

        if keyboard[Key::L] == ButtonState::Pressed {
            use super::license_panel::make_license_panel;
            actions.push(PanelAction::AddPanelAbove(Box::new(make_license_panel())));
        }

        game.world.execute(|world| {
            let map = &mut world.map;
            let player = &mut world.player;
            let camera = &mut world.camera;

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

        Ok(actions)
    }
}
