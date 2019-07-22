use super::*;

use std::collections::HashMap;

use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, Font, FontStyle, Image},
    lifecycle::{State, Window},
    load_file, Future,
};
use specs::Builder;

use loader::Loadable;

use resources::*;
use world::{Map, MapGenerationParams, TilePos, WorldState};

pub struct MainState {
    world: World,
    assets: GameAssets,
}

pub struct GameAssets {
    world_params: Asset<MapGenerationParams>,
    world_state_ready: resources::InitializationState,

    tileset: Asset<HashMap<char, Image>>,

    controls_image: ControlsImages,

    // TODO: have to pass in a reference to game assets in the update system
    // since assets aren't sync or send
    dialogue_assets: Option<DialogueAssets>,
}

pub struct ControlsImages {
    pub hack: Asset<Image>,
    pub talk: Asset<Image>,
    pub repair: Asset<Image>,
}

pub struct DialogueAssets {
    main_text: Asset<Image>,
    option_assets: Vec<DialogueOptionAsset>,
}

pub struct DialogueOptionAsset {
    selected_text: Asset<Image>,
    unselected_text: Asset<Image>,
}

pub const FONT_MONONOKI_PATH: &str = "fonts/mononoki/mononoki-Regular.ttf";
pub const FONT_SQUARE_PATH: &str = "fonts/square/square.ttf";

// TODO: autogen this list somehow
pub const ALL_GAME_GLYPHS: &str = "QWERTYUIOPASDFGHJKLZXCVBNMqwertyuiopasdfghjklzxcvbnm,.;:?%* █aAdD@I:`0123456789";

fn render_mononoki(text: String, size: f32, color: Color) -> Asset<Image> {
    Asset::new(Font::load(FONT_MONONOKI_PATH).and_then(move |font| font.render(&text, &FontStyle::new(size, color))))
}

fn make_assets() -> GameAssets {
    let world_params: Asset<MapGenerationParams> = Asset::new(load_file("config/map_params.ron").and_then(move |bytes| {
        let map_gen_params = ron::de::from_bytes(&bytes).expect("Should deserialize");
        Ok(map_gen_params)
    }));

    let render_params = GameMapRenderParams::default();
    let tile_size_px = render_params.tile_size_px;

    let tileset = Asset::new(Font::load(game_state::FONT_SQUARE_PATH).and_then(move |text| {
        let tiles = text
            .render(ALL_GAME_GLYPHS, &FontStyle::new(tile_size_px.y, Color::WHITE))
            .expect("Could not render the font tileset");

        let mut tileset = HashMap::new();
        for (index, glyph) in ALL_GAME_GLYPHS.chars().enumerate() {
            let pos = (index as i32 * tile_size_px.x as i32, 0);
            let tile = tiles.subimage(Rectangle::new(pos, tile_size_px));
            tileset.insert(glyph, tile);
        }
        Ok(tileset)
    }));

    let controls_image = ControlsImages {
        hack: render_mononoki("[H]ack".to_owned(), 18.0, Color::BLACK),
        repair: render_mononoki("[R]epair".to_owned(), 18.0, Color::BLACK),
        talk: render_mononoki("[T]alk".to_owned(), 18.0, Color::BLACK),
    };

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

fn setup<'a, T: System<'a>>(sys: &mut T, world: &mut World) {
    sys.setup(world);
}

fn run_now<'a, T>(sys: &mut T, world: &'a mut World)
where
    T: for<'b> System<'b>,
{
    sys.run_now(world);
    world.maintain();
}

macro_rules! systems {
    ($method_name:ident, $world_name:ident) => {
        // No-op systems for setting up render resources
        $method_name(&mut systems::CharsRendererSetup, $world_name);
        $method_name(&mut systems::ControlsRendererSetup, $world_name);
        $method_name(&mut systems::OxygenOverlaySetup, $world_name);

        // important update systems; order matters, be careful
        // player doing stuff
        timed!("DialogueControl", $method_name(&mut systems::DialogueControlSystem, $world_name));
        timed!("PlayerMove", $method_name(&mut systems::PlayerMoveSystem, $world_name));
        timed!("ToggleControl", $method_name(&mut systems::ToggleControlSystem, $world_name));
        timed!("ToggleHack", $method_name(&mut systems::ToggleHackSystem, $world_name));
        timed!("ToggleTalk", $method_name(&mut systems::ToggleTalkSystem, $world_name));

        timed!(
            "HackCallbackHandlerSystem",
            $method_name(&mut systems::HackCallbackHandlerSystem, $world_name)
        );

        timed!(
            "TalkCallbackHandlerSystem",
            $method_name(&mut systems::TalkCallbackHandlerSystem, $world_name)
        );

        // non-players doing stuff
        timed!("NpcMoves", $method_name(&mut systems::NpcMoveSystem, $world_name));

        // various updates of inanimates
        timed!("Breathe", $method_name(&mut systems::BreatheSystem, $world_name));
        timed!("DoorOpen", $method_name(&mut systems::DoorOpenSystem, $world_name));
        timed!("OxygenSpread", $method_name(&mut systems::OxygenSpreadSystem, $world_name));

        timed!(
            "DialogueUpdateSystem",
            $method_name(&mut systems::DialogueUpdateSystem, $world_name)
        );

        // bookkeeping stuff after things stop changing
        timed!("SpaceInserter", $method_name(&mut systems::FakeSpaceInserterSystem, $world_name)); // before vis, after stuff moves
        timed!("Visibility", $method_name(&mut systems::VisibilitySystem, $world_name));

        // end of turn upkeep
        timed!("PlayerNotMoved", $method_name(&mut systems::PlayerNotMoved, $world_name));

        timed!("SaveGame", $method_name(&mut systems::SerializeSystem, $world_name));
        timed!("LoadGame", $method_name(&mut systems::DeserializeSystem, $world_name));

        timed!(
            "GameIsQuitChecker",
            $method_name(&mut systems::GameIsQuitCheckerSystem, $world_name)
        );
        timed!("CallbackChecker", $method_name(&mut systems::CallbackCheckerSystem, $world_name));
    };
}

fn setup_systems(world: &mut World) {
    timed!("Set up all systems", {
        systems!(setup, world);
    });
}

fn run_systems(world: &mut World) {
    timed!("Run all systems", {
        systems!(run_now, world);
    });
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
                        world.insert::<WorldState>(world_state);
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
                    if assets.dialogue_assets.is_loaded()? {
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
                                let new_assets = make_dialogue_assets(state);
                                let new_state = {
                                    if assets.dialogue_assets.is_loaded()? {
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
            && assets.tileset.is_loaded()?
            && assets.controls_image.is_loaded()?;

        Ok(loaded)
    }
}

impl Loadable for DialogueAssets {
    fn is_loaded(&mut self) -> QsResult<bool> {
        let mut out = true;

        out &= self.main_text.is_loaded()?;
        out &= self.option_assets.is_loaded()?;

        Ok(out)
    }
}

impl Loadable for DialogueOptionAsset {
    fn is_loaded(&mut self) -> QsResult<bool> {
        let mut out = true;

        out &= self.selected_text.is_loaded()?;
        out &= self.unselected_text.is_loaded()?;

        Ok(out)
    }
}

impl Loadable for ControlsImages {
    fn is_loaded(&mut self) -> QsResult<bool> {
        let mut out = true;

        out &= self.hack.is_loaded()?;
        out &= self.talk.is_loaded()?;
        out &= self.repair.is_loaded()?;

        Ok(out)
    }
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
        world.register::<components::SaveComponent>();
        world.insert(components::SaveComponentAllocator::new());

        let assets = make_assets();
        setup_systems(&mut world);

        let _player = world
            .create_entity()
            .with(components::HasPosition {
                // TODO: somehow configure this so it's guaranteed to be a good spot
                position: TilePos { x: 15, y: 15 },
            })
            .with(components::BlocksMovement)
            .with(components::CharRender {
                glyph: '@',
                z_level: components::ZLevel::OnFloor,
                bg_color: CLEAR,
                fg_color: quicksilver::graphics::Color::MAGENTA,
                disabled: false,
            })
            .with(components::OpensDoors)
            .with(components::Visible {
                visibility: world::VisibilityType::CurrentlyVisible,
                memorable: false,
            })
            .with(components::Breathes::default())
            .with(components::CanSuffocate::Player)
            .with(components::Player {})
            .marked::<components::SaveComponent>()
            .build();

        let _camera = world
            .create_entity()
            .with(components::HasPosition {
                position: TilePos { x: 15, y: 15 },
            })
            .with(components::Camera { x_rad: 15, y_rad: 15 })
            .marked::<components::SaveComponent>()
            .build();

        systems::start::StartGameSystem.run_now(&world);

        Ok(MainState { world, assets })
    }

    fn update(&mut self, window: &mut Window) -> QsResult<()> {
        if !(self.ensure_initialized()?) {
            return Ok(());
        }

        self.world.insert::<quicksilver::input::Keyboard>(*window.keyboard());

        if self.world.read_resource::<GameIsQuit>().0 {
            window.close();
            return Ok(());
        }

        run_systems(&mut self.world);

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> QsResult<()> {
        if !(self.ensure_initialized()?) {
            return Ok(());
        }

        timed!("Rendering", {
            window.set_blend_mode(quicksilver::graphics::BlendMode::Additive)?;

            let bg_color = Color::from_hex("556887");
            window.clear(bg_color)?;

            // Because of borrow lifetimes, these systems can't persist
            // so we just call em manually, it works great
            systems::CharsRenderer {
                window,
                tileset: &mut self.assets.tileset,
            }
            .run_now(&self.world);

            systems::OxygenOverlayRenderer { window }.run_now(&self.world);

            systems::ControlsRenderer {
                window,
                controls_image: &mut self.assets.controls_image,
                tileset: &mut self.assets.tileset,
            }
            .run_now(&self.world);

            window.flush()?;

            if let Some(dialogue_assets) = self.assets.dialogue_assets.as_mut() {
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
                .run_now(&self.world);
            }
        });

        Ok(())
    }
}
