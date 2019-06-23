use super::*;

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
use world::{Map, MapGenerationParams, TilePos, WorldState};

pub struct MainState {
    world: World,
    assets: GameAssets,

    update_dispatcher: Dispatcher<'static, 'static>,
}

pub struct GameAssets {
    world_params: Asset<MapGenerationParams>,
    world_state_ready: InitializationState,

    tileset: Asset<HashMap<char, Image>>,

    controls_image: Asset<Image>,

    // TODO: have to pass in a reference to game assets in the update system
    // since assets aren't sync or send
    dialogue_assets: Option<DialogueAssets>,
}

pub struct DialogueAssets {
    main_text: Asset<Image>,
    option_assets: Vec<DialogueOptionAsset>,
}

pub struct DialogueOptionAsset {
    selected_text: Asset<Image>,
    unselected_text: Asset<Image>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum InitializationState {
    NotStarted,
    Started,
    Finished,
}

#[derive(Clone, Debug)]
pub struct DialogueStateResource {
    pub is_initialized: InitializationState,
    pub state: Option<DialogueState>,
}

impl Default for DialogueStateResource {
    fn default() -> Self {
        DialogueStateResource {
            // if dialogue is off, state is nothing, it's fine
            is_initialized: InitializationState::Finished,
            state: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct DialogueState {
    pub main_text: String,
    pub selected_index: usize,
    pub options: Vec<DialogueOptionState>,
}

#[derive(Clone, Debug)]
pub struct DialogueOptionState {
    pub selected_text: String,
    pub unselected_text: String,
    pub callbacks: Vec<DialogueCallback>, // TODO: somehow need to track the callbacks (?)
}

#[derive(Clone, Debug)]
pub enum DialogueCallback {
    EndDialogue,
    QuitGame,
}

// TODO: move stuff into a resources module

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum KeyboardFocus {
    GameMap,
    Dialogue,
}

impl Default for KeyboardFocus {
    fn default() -> Self {
        KeyboardFocus::GameMap
    }
}

#[derive(Debug, Copy, Clone)]
pub struct GameIsQuit(pub bool);

impl Default for GameIsQuit {
    fn default() -> Self {
        GameIsQuit(false)
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

pub const FONT_MONONOKI_PATH: &str = "fonts/mononoki/mononoki-Regular.ttf";
pub const FONT_SQUARE_PATH: &str = "fonts/square/square.ttf";

fn make_assets() -> GameAssets {
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

    GameAssets {
        world_params,
        world_state_ready: InitializationState::Started,

        tileset,
        controls_image,

        dialogue_assets: None,
    }
}

// mononoki
fn make_text_image(text: String, color: Color) -> Asset<Image> {
    Asset::new(Font::load(game_state::FONT_MONONOKI_PATH).and_then(move |font| font.render(&text, &FontStyle::new(18.0, color))))
}

fn make_update_dispatcher(world: &mut World) -> Dispatcher<'static, 'static> {
    let mut render_dispatcher: Dispatcher = DispatcherBuilder::new()
        // fake/noop systems; used to make sure certain components are set up
        // since the actual render system isn't dispatched
        .with(systems::CharsRendererSetup, "chars_render", &[])
        .with(systems::ControlsRendererSetup, "controls_render", &[])
        .build();

    // we don't need to keep this dispatcher, just call setup on it
    render_dispatcher.setup(&mut world.res);

    let mut out = DispatcherBuilder::new()
        // note: we run everything in sequence, so the dependencies don't matter
        // TODO: we need all these things to check for keyboard focus (currently they don't)
        .with(systems::PlayerMoveSystem, "player_move", &[])
        .with(systems::VisibilitySystem, "visibility", &[])
        .with(systems::ToggleControlSystem, "toggle_controls", &[])
        .with(systems::DialogueControlSystem, "dialogue_controls", &[])
        .build();

    out.setup(&mut world.res);

    out
}

impl MainState {
    fn ensure_initialized(&mut self) -> QsResult<bool> {
        let assets = &mut self.assets;
        let world = &mut self.world;

        let world_state_ready = {
            match assets.world_state_ready {
                InitializationState::Finished => InitializationState::Finished,
                InitializationState::NotStarted => unimplemented!(), // TODO: make world state load like everything else?
                InitializationState::Started => {
                    let mut init = InitializationState::Started;
                    assets.world_params.execute(|params| {
                        let map = Map::make_random(&params, world);
                        let world_state = WorldState::new(map);
                        world.add_resource::<WorldState>(world_state);
                        init = InitializationState::Finished;
                        Ok(())
                    })?;

                    init
                }
            }
        };

        assets.world_state_ready = world_state_ready;

        let dialogue_state_resource: &mut DialogueStateResource = &mut world.write_resource::<DialogueStateResource>();
        let dialogue_ready = {
            match dialogue_state_resource.is_initialized {
                InitializationState::Finished => InitializationState::Finished,
                InitializationState::Started => {
                    if check_dialogue_assets_loaded(assets.dialogue_assets.as_mut().unwrap())? {
                        InitializationState::Finished
                    } else {
                        InitializationState::Started
                    }
                }
                InitializationState::NotStarted => {
                    let (new_state, new_assets) = {
                        match dialogue_state_resource.state.as_ref() {
                            None => (InitializationState::Finished, None),
                            Some(state) => {
                                let mut new_assets = make_dialogue_assets(state);
                                let new_state = {
                                    if check_dialogue_assets_loaded(&mut new_assets)? {
                                        InitializationState::Finished
                                    } else {
                                        InitializationState::Started
                                    }
                                };
                                (new_state, Some(new_assets))
                            }
                        }
                    };
                    assets.dialogue_assets = new_assets;
                    new_state
                }
            }
        };

        dialogue_state_resource.is_initialized = dialogue_ready;

        let loaded = world_state_ready == InitializationState::Finished
            && dialogue_ready == InitializationState::Finished
            && is_loaded(&mut assets.tileset)?
            && is_loaded(&mut assets.controls_image)?;

        Ok(loaded)
    }
}

fn check_dialogue_assets_loaded(assets: &mut DialogueAssets) -> QsResult<bool> {
    let mut out = is_loaded(&mut assets.main_text)?;
    for doa in &mut assets.option_assets {
        out &= is_loaded(&mut doa.selected_text)?;
        out &= is_loaded(&mut doa.unselected_text)?;
    }
    Ok(out)
}

fn make_dialogue_assets(dialogue_state: &DialogueState) -> DialogueAssets {
    let main_text = make_text_image(dialogue_state.main_text.clone(), Color::WHITE);

    let option_assets = dialogue_state
        .options
        .iter()
        .map(|dialogue_option| {
            let selected_text = make_text_image(dialogue_option.selected_text.clone(), Color::YELLOW);
            let unselected_text = make_text_image(dialogue_option.unselected_text.clone(), Color::WHITE);

            DialogueOptionAsset {
                selected_text,
                unselected_text,
            }
        })
        .collect();

    DialogueAssets { main_text, option_assets }
}

impl State for MainState {
    fn new() -> QsResult<Self> {
        let mut world = World::new();

        let assets = make_assets();

        let update_dispatcher = make_update_dispatcher(&mut world);

        // TODO: probably shouldn't need this?
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

        Ok(MainState {
            world,
            assets,
            update_dispatcher,
        })
    }

    fn update(&mut self, window: &mut Window) -> QsResult<()> {
        if !(self.ensure_initialized()?) {
            return Ok(());
        }

        self.world.add_resource::<quicksilver::input::Keyboard>(*window.keyboard());

        if self.world.read_resource::<GameIsQuit>().0 {
            window.close();
            return Ok(());
        }

        self.update_dispatcher.run_now(&self.world.res);

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> QsResult<()> {
        if !(self.ensure_initialized()?) {
            return Ok(());
        }

        let bg_color = Color::from_hex("556887");
        window.clear(bg_color)?;

        // Because of borrow lifetimes, these systems can't persist
        // so we just call em manually, it works great
        systems::CharsRenderer {
            window,
            tileset: &mut self.assets.tileset,
        }
        .run_now(&self.world.res);

        systems::ControlsRenderer {
            window,
            controls_image: &mut self.assets.controls_image,
        }
        .run_now(&self.world.res);

        window.flush()?;

        if let Some(dialogue_assets) = self.assets.dialogue_assets.as_mut() {
            // TODO: dim the backdrop somehow?
            window.draw(
                &Rectangle {
                    pos: Vector::new(0, 0),
                    size: window.screen_size(),
                },
                Col(Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.7,
                }),
            );

            let selected_index = self
                .world
                .read_resource::<DialogueStateResource>()
                .state
                .as_ref()
                .expect("If dialogue assets are defined, state should be too")
                .selected_index;

            let mut dialogue_images = Vec::with_capacity(dialogue_assets.option_assets.len() + 1);
            dialogue_images.push(&mut dialogue_assets.main_text);
            dialogue_assets.option_assets.iter_mut().enumerate().for_each(|(i, option)| {
                let image = if i == selected_index {
                    &mut option.selected_text
                } else {
                    &mut option.unselected_text
                };
                dialogue_images.push(image);
            });

            systems::CenteredVerticalImagesRenderer {
                window,
                images: &mut dialogue_images,
                bg_color: Color {
                    r: 0.3,
                    g: 0.6,
                    b: 0.5,
                    a: 1.0,
                },
                outside_padding: Vector::new(30.0, 30.0),
                internal_padding: Vector::new(20.0, 20.0),
            }
            .run_now(&self.world.res);
        }

        Ok(())
    }
}

pub fn is_loaded<T>(asset: &mut Asset<T>) -> QsResult<bool> {
    let mut is_loaded = false;
    asset.execute(|_t| {
        is_loaded = true;
        Ok(())
    })?;
    Ok(is_loaded)
}

pub fn all_loaded<'a, T>(assets: &mut [&'a mut Asset<T>]) -> QsResult<bool> {
    for asset in assets.iter_mut() {
        let good = is_loaded(asset)?;
        if !good {
            return Ok(false);
        }
    }

    Ok(true)
}
