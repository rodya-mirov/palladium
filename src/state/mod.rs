use std::collections::HashMap;
use std::ops::Add;

use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, Font, FontStyle, Image},
    load_file,
    prelude::*,
    Future,
};

use crate::maps::{Map, SquareType};
use crate::panel::{Panel, PanelAction};
use crate::visibility::refresh_visibility;
use crate::QsResult;

pub struct Game {
    pub map: Asset<Map>,

    pub camera: CameraInfo,

    pub player: Player,

    pub controls_pane: ControlsPane,

    pub is_quit: bool,

    panels: Vec<Box<dyn Panel>>,

    pub tileset: Asset<HashMap<char, Image>>,
    pub tile_size_px: Vector,
}

pub struct Player {
    pub tile_pos: TilePos,
}

impl Player {
    pub fn try_move(&mut self, translation: TilePos, map: &Map) -> bool {
        let new_pos = TilePos {
            x: self.tile_pos.x + translation.x,
            y: self.tile_pos.y + translation.y,
        };
        let to_move_to = map.get_square(new_pos.x, new_pos.y);

        let can_move = match to_move_to.square_type {
            SquareType::Void => false,
            SquareType::Wall => false,
            SquareType::Open => false,

            SquareType::Floor => true,
            SquareType::Door => true,
        };

        if can_move {
            self.tile_pos = new_pos;
        }

        can_move
    }
}

pub struct ControlsPane {
    pub controls_image: Asset<Image>,
    pub show_controls_image: bool,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct CameraInfo {
    pub x_min: i32,
    pub x_max: i32,

    pub y_min: i32,
    pub y_max: i32,
}

impl CameraInfo {
    pub fn translate(&mut self, translation: TilePos) {
        self.x_min += translation.x;
        self.x_max += translation.x;
        self.y_min += translation.y;
        self.y_max += translation.y;
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct TilePos {
    pub x: i32,
    pub y: i32,
}

impl Add<TilePos> for TilePos {
    type Output = TilePos;

    fn add(self, rhs: TilePos) -> TilePos {
        TilePos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Game {
    pub fn quit(&mut self) {
        self.is_quit = true;
    }

    fn process_panel_actions(&mut self, actions_by_panel: Vec<Vec<PanelAction>>) {
        let mut new_panels = Vec::new();

        for panel_actions in actions_by_panel {
            let old_panel = self.panels.remove(0);

            let mut keep_old = true;
            let mut next_panel_index = new_panels.len();
            for panel_action in panel_actions {
                match panel_action {
                    PanelAction::CloseCurrentPanel => {
                        keep_old = false;
                    }
                    PanelAction::AddPanelAbove(new_panel) => {
                        new_panels.push(new_panel);
                    }
                    PanelAction::AddPanelBehind(new_panel) => {
                        new_panels.insert(next_panel_index, new_panel);
                        next_panel_index += 1;
                    }
                }
            }
            if keep_old {
                new_panels.insert(next_panel_index, old_panel);
            }
        }

        self.panels = new_panels;
    }
}

impl State for Game {
    /// Load assets and so on
    fn new() -> QsResult<Game> {
        make_new_game()
    }

    fn update(&mut self, window: &mut Window) -> QsResult<()> {
        if self.panels.is_empty() {
            window.close();
            return Ok(());
        }

        let mut actions_by_panel = Vec::with_capacity(self.panels.len());

        let mut actual_panels = std::mem::replace(&mut self.panels, Vec::new());
        let last_ind = actual_panels.len() - 1;
        for (i, panel) in actual_panels.iter_mut().enumerate() {
            // TODO: if there's an error this will leave all the panels dead, which will be bad
            let panel_actions = panel.update_self(self, i == last_ind)?;
            actions_by_panel.push(panel_actions);
        }

        let mut kb_update_actions = actual_panels
            .get_mut(last_ind)
            .expect("This should always exist because the vec is nonempty")
            .do_key_input(self, window.keyboard())?;

        actions_by_panel
            .get_mut(last_ind)
            .expect("This should always exist")
            .append(&mut kb_update_actions);

        std::mem::replace(&mut self.panels, actual_panels);

        self.process_panel_actions(actions_by_panel);

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> QsResult<()> {
        let mut actions_by_panel = Vec::with_capacity(self.panels.len());
        let mut actual_panels = std::mem::replace(&mut self.panels, Vec::new());
        for panel in actual_panels.iter_mut() {
            window.flush()?;
            let panel_render_actions = panel.render_self(self, window)?;
            actions_by_panel.push(panel_render_actions);
        }
        std::mem::replace(&mut self.panels, actual_panels);

        self.process_panel_actions(actions_by_panel);
        Ok(())
    }
}

pub const FONT_MONONOKI_PATH: &str = "fonts/mononoki/mononoki-Regular.ttf";
pub const FONT_SQUARE_PATH: &str = "fonts/square/square.ttf";

fn make_new_game() -> QsResult<Game> {
    // TODO: autogen this list somehow
    let game_glyphs = "* █d@";

    let tile_size_px = Vector::new(20, 20);

    let controls_image = Asset::new(Font::load(FONT_MONONOKI_PATH).and_then(move |font| {
        let controls = vec!["[Q]uit", "[C]ontrols", "[L]icenses"];

        font.render(&(controls.join("\n")), &FontStyle::new(18.0, Color::BLACK))
    }));

    let tileset = Asset::new(Font::load(FONT_SQUARE_PATH).and_then(move |text| {
        let tiles = text
            .render(game_glyphs, &FontStyle::new(tile_size_px.y, Color::WHITE))
            .expect("Could not render the font tileset");

        let mut tileset = HashMap::new();
        for (index, glyph) in game_glyphs.chars().enumerate() {
            let pos = (index as i32 * tile_size_px.x as i32, 0);
            let tile = tiles.subimage(Rectangle::new(pos, tile_size_px));
            tileset.insert(glyph, tile);
        }
        Ok(tileset)
    }));

    // TODO: make this square guaranteed to not be empty
    let player_pos = TilePos { x: 15, y: 15 };

    let player = Player { tile_pos: player_pos };

    let map: Asset<Map> = Asset::new(load_file("config/map_params.ron").and_then(move |bytes| {
        let map_gen_params = ron::de::from_bytes(&bytes).expect("Should deserialize");
        let mut map: Map = Map::make_random(&map_gen_params);
        refresh_visibility(player_pos, &mut map, 1000); // TODO: vis range
        Ok(map)
    }));

    let camera = CameraInfo {
        x_min: 0,
        x_max: 30,

        y_min: 0,
        y_max: 30,
    };

    Ok(Game {
        controls_pane: ControlsPane {
            controls_image,
            show_controls_image: true,
        },

        camera,
        map,
        player,
        is_quit: false,
        panels: vec![Box::new(crate::panel::game_panel::GamePanel::new())],

        tile_size_px,
        tileset,
    })
}

fn make_mononoki_font_image() -> Asset<Image> {
    Asset::new(Font::load(FONT_MONONOKI_PATH).and_then(|font| {
        font.render(
            "Mononoki font by Matthias Tellen, terms: SIL Open Font License 1.1",
            &FontStyle::new(20.0, Color::BLACK),
        )
    }))
}

fn make_square_font_image() -> Asset<Image> {
    Asset::new(Font::load(FONT_SQUARE_PATH).and_then(|font| {
        font.render(
            "Square font by wouter van oortmerssen, terms: CC BY 3.0",
            &FontStyle::new(20.0, Color::BLACK),
        )
    }))
}