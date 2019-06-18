use std::collections::HashMap;

use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, Font, FontStyle, Image},
    load_file,
    prelude::*,
    Future,
};

use crate::maps::Map;
use crate::panel::{Panel, PanelAction};
use crate::world::World;
use crate::QsResult;

pub struct Game {
    pub world: Asset<World>,

    pub controls_pane: ControlsPane,

    pub is_quit: bool,

    panels: Vec<Box<dyn Panel>>,

    pub tileset: Asset<HashMap<char, Image>>,
    pub tile_size_px: Vector,
}

pub struct ControlsPane {
    pub controls_image: Asset<Image>,
    pub show_controls_image: bool,
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
    let game_glyphs = "* █d@I`";

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

    let world: Asset<World> = Asset::new(load_file("config/map_params.ron").and_then(move |bytes| {
        let map_gen_params = ron::de::from_bytes(&bytes).expect("Should deserialize");
        let map = Map::make_random(&map_gen_params);
        let world = World::new(map);
        Ok(world)
    }));

    Ok(Game {
        controls_pane: ControlsPane {
            controls_image,
            show_controls_image: true,
        },

        world,

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
