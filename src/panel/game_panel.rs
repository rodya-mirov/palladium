//! The main game panel, for the map and stuff (the "actual game")

use std::collections::HashMap;

use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, Font, FontStyle, Image},
    lifecycle::Window,
    load_file,
    prelude::*,
    Future,
};
use specs::{Builder, Dispatcher, DispatcherBuilder, RunNow, World};

use super::*;

use game_state::is_loaded;
use panel::Panel;
use world::{Map, MapGenerationParams, TilePos, WorldState};

#[derive(Debug, Copy, Clone)]
pub struct GameIsQuit(pub bool);

impl Default for GameIsQuit {
    fn default() -> Self {
        GameIsQuit(false)
    }
}

pub struct GamePanel {
    world_params: Asset<MapGenerationParams>,
    world_state_initiated: bool,

    tileset: Asset<HashMap<char, Image>>,

    controls_image: Asset<Image>,

    update_dispatcher: Dispatcher<'static, 'static>,
    input_dispatcher: Dispatcher<'static, 'static>,
    render_dispatcher: Dispatcher<'static, 'static>,
}

impl GamePanel {
    pub fn new(world: &mut World) -> GamePanel {
        let world_params: Asset<MapGenerationParams> = Asset::new(load_file("config/map_params.ron").and_then(move |bytes| {
            let map_gen_params = ron::de::from_bytes(&bytes).expect("Should deserialize");
            Ok(map_gen_params)
        }));

        // TODO: autogen this list somehow
        let game_glyphs = "* █d@I`";
        let render_params = GameMapRenderParams::default();
        let tile_size_px = render_params.tile_size_px;

        let tileset = Asset::new(Font::load(game_state::FONT_SQUARE_PATH).and_then(move |text| {
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

        let controls_image = Asset::new(Font::load(game_state::FONT_MONONOKI_PATH).and_then(move |font| {
            let controls = vec!["[Q]uit", "[C]ontrols", "[L]icenses"];

            font.render(&(controls.join("\n")), &FontStyle::new(18.0, Color::BLACK))
        }));

        let mut update_dispatcher = DispatcherBuilder::new().build();

        let mut input_dispatcher = DispatcherBuilder::new()
            // note: we run everything in sequence, so the dependencies don't matter
            .with(systems::PlayerMoveSystem, "player_move", &[])
            .with(systems::VisibilitySystem, "visibility", &[])
            .with(systems::ToggleControlSystem, "toggle_controls", &[])
            .build();

        let mut render_dispatcher: Dispatcher = DispatcherBuilder::new()
            // fake/noop systems; used to make sure certain components are set up
            // since the actual render system isn't dispatched
            .with(systems::CharsRendererSetup, "chars_render", &[])
            .with(systems::ControlsRendererSetup, "controls_render", &[])
            .build();

        update_dispatcher.setup(&mut world.res);
        input_dispatcher.setup(&mut world.res);
        render_dispatcher.setup(&mut world.res);

        world.add_resource(GameIsQuit::default());

        let _player = world
            .create_entity()
            .with(components::HasPosition {
                // TODO: somehow configure this so it's guaranteed to be a good spot
                position: TilePos { x: 15, y: 15 },
            })
            .with(components::CharRender {
                glyph: '@',
                fg_color: quicksilver::graphics::Color::MAGENTA,
            })
            .with(components::Visible {
                visibility: world::VisibilityType::CurrentlyVisible,
                occludes: false,
                memorable: false,
            })
            .with(components::Player {})
            .build();

        let _camera = world
            .create_entity()
            .with(components::HasPosition {
                position: TilePos { x: 15, y: 15 },
            })
            .with(components::Camera { x_rad: 15, y_rad: 15 })
            .build();

        GamePanel {
            world_params,
            world_state_initiated: false,

            tileset,

            controls_image,

            update_dispatcher,
            input_dispatcher,
            render_dispatcher,
        }
    }

    fn ensure_initialized(&mut self, world: &mut World) -> QsResult<bool> {
        if !self.world_state_initiated {
            let (init, world) = (&mut self.world_state_initiated, world);
            self.world_params.execute(|params| {
                let map = Map::make_random(&params, world);
                let world_state = WorldState::new(map);
                world.add_resource::<WorldState>(world_state);
                *init = true;
                Ok(())
            })?;

            if !self.world_state_initiated {
                return Ok(false);
            }
        }

        let loaded = is_loaded(&mut self.tileset)? && is_loaded(&mut self.controls_image)?;

        Ok(loaded)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct GameMapDisplayOptions {
    pub display_controls_pane: bool,
}

impl Default for GameMapDisplayOptions {
    fn default() -> Self {
        GameMapDisplayOptions {
            display_controls_pane: true,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GameMapRenderParams {
    pub font_width: f32,
    pub map_offset: Vector,
    pub tile_size_px: Vector,
    pub controls_image_offset_px: Vector,
}

impl Default for GameMapRenderParams {
    fn default() -> Self {
        GameMapRenderParams {
            font_width: 20.0,
            map_offset: Vector::new(50.0, 50.0),
            tile_size_px: Vector::new(20.0, 20.0),
            controls_image_offset_px: Vector::new(30.0, 30.0),
        }
    }
}

impl Panel for GamePanel {
    fn update_self(&mut self, world: &mut World, _is_active: bool) -> PanelResult {
        if !self.ensure_initialized(world)? {
            return Ok(Vec::new());
        }

        self.update_dispatcher.dispatch_seq(&world.res);

        let is_quit = world.read_resource::<GameIsQuit>().0;
        if is_quit {
            Ok(vec![PanelAction::CloseCurrentPanel])
        } else {
            Ok(Vec::new())
        }
    }

    fn render_self(&mut self, world: &mut World, window: &mut Window) -> PanelResult {
        if !self.ensure_initialized(world)? {
            return Ok(Vec::new());
        }

        let bg_color = Color::from_hex("556887");
        window.clear(bg_color)?;

        self.render_dispatcher.run_now(&world.res);

        // Because of borrow lifetimes, these systems can't persist
        // so we just call em manually, it works great
        systems::CharsRenderer {
            window,
            tileset: &mut self.tileset,
        }
        .run_now(&world.res);

        systems::ControlsRenderer {
            window,
            controls_image: &mut self.controls_image,
        }
        .run_now(&world.res);

        Ok(Vec::new())
    }

    fn do_key_input(&mut self, world: &mut World, keyboard: &Keyboard) -> PanelResult {
        if !self.ensure_initialized(world)? {
            return Ok(Vec::new());
        }

        self.input_dispatcher.dispatch_seq(&world.res);

        let mut actions = Vec::new();
        if keyboard[Key::Q] == ButtonState::Pressed {
            actions.push(PanelAction::AddPanelAbove(Box::new(make_quit_panel())));
        } else if keyboard[Key::L] == ButtonState::Pressed {
            actions.push(PanelAction::AddPanelAbove(Box::new(make_license_panel())));
        }
        Ok(actions)
    }
}
