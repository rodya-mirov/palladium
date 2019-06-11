use super::*;

use std::collections::HashMap;
use std::ops::Add;

use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, Font, FontStyle, Image},
    lifecycle::{Asset, State, Window},
    load_file,
    prelude::*,
    Future,
};

use crate::maps::{Map, Square, SquareType, VisibilityType};

mod visibility;

const PLAYER_CHAR: char = '@';

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct TilePos {
    x: i32,
    y: i32,
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

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct CameraInfo {
    x_min: i32,
    x_max: i32,

    y_min: i32,
    y_max: i32,
}

impl CameraInfo {
    fn translate(&mut self, translation: TilePos) {
        self.x_min += translation.x;
        self.x_max += translation.x;
        self.y_min += translation.y;
        self.y_max += translation.y;
    }
}

struct Player {
    tile_pos: TilePos,
}

fn with_vis(color: Color, visibility: VisibilityType) -> Color {
    let alpha = match visibility {
        VisibilityType::NotSeen => 0.0,
        VisibilityType::Remembered => 0.3,
        VisibilityType::CurrentlyVisible => 1.0,
    };

    color.with_alpha(alpha)
}

impl Player {
    fn try_move(&mut self, translation: TilePos, map: &Map) -> bool {
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

pub struct Game {
    map: Asset<Map>,

    camera: CameraInfo,

    player: Player,

    controls_pane: ControlsPane,
    title: Asset<Image>,
    mononoki_font_info: Asset<Image>,
    square_font_info: Asset<Image>,

    tileset: Asset<HashMap<char, Image>>,
    tile_size_px: Vector,
}

struct ControlsPane {
    controls_image: Asset<Image>,
    show_controls_image: bool,
}

impl State for Game {
    // load assets and etc.
    fn new() -> QsResult<Game> {
        let font_mononoki = "fonts/mononoki/mononoki-Regular.ttf";
        let font_square = "fonts/square/square.ttf";

        let title = Asset::new(Font::load(font_mononoki).and_then(|font| font.render("Palladium", &FontStyle::new(72.0, Color::BLACK))));

        let mononoki_font_info = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render(
                "Mononoki font by Matthias Tellen, terms: SIL Open Font License 1.1",
                &FontStyle::new(20.0, Color::BLACK),
            )
        }));
        let square_font_info = Asset::new(Font::load(font_square).and_then(|font| {
            font.render(
                "Square font by wouter van oortmerssen, terms: CC BY 3.0",
                &FontStyle::new(20.0, Color::BLACK),
            )
        }));

        // TODO: autogen this list somehow
        let game_glyphs = "* █d@";

        let tile_size_px = Vector::new(20, 20);

        let controls_image = Asset::new(Font::load(font_square).and_then(move |font| {
            let controls = vec!["Quit", "Controls", "Map"];

            font.render(&(controls.join("\n")), &FontStyle::new(12.0, Color::BLACK))
        }));

        let tileset = Asset::new(Font::load(font_square).and_then(move |text| {
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
            visibility::refresh_visibility(player_pos, &mut map, 1000); // TODO: vis range
            Ok(map)
        }));

        let camera = CameraInfo {
            x_min: 0,
            x_max: 30,

            y_min: 0,
            y_max: 30,
        };

        Ok(Game {
            title,
            mononoki_font_info,
            square_font_info,
            controls_pane: ControlsPane {
                controls_image,
                show_controls_image: true,
            },

            camera,
            map,
            player,

            tile_size_px,
            tileset,
        })
    }

    fn update(&mut self, window: &mut Window) -> QsResult<()> {
        use quicksilver::input::ButtonState::*;

        let map = &mut self.map;
        let player = &mut self.player;
        let camera = &mut self.camera;

        let keyboard = window.keyboard();

        if keyboard[Key::C] == Pressed {
            self.controls_pane.show_controls_image = !self.controls_pane.show_controls_image;
        }

        if keyboard[Key::Q] == Pressed {
            window.close();
            return Ok(());
        }

        map.execute(|map| {
            let player_move = if keyboard[Key::Left] == Pressed {
                Some(TilePos { x: -1, y: 0 })
            } else if keyboard[Key::Up] == Pressed {
                Some(TilePos { x: 0, y: -1 })
            } else if keyboard[Key::Down] == Pressed {
                Some(TilePos { x: 0, y: 1 })
            } else if keyboard[Key::Right] == Pressed {
                Some(TilePos { x: 1, y: 0 })
            } else {
                None
            };

            if let Some(player_move) = player_move {
                let can_move = player.try_move(player_move, map);
                if can_move {
                    camera.translate(player_move);
                    visibility::refresh_visibility(player.tile_pos, map, 1000);
                }
            }

            Ok(())
        })?;

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> QsResult<()> {
        let bg_color = Color::from_hex("556887");
        window.clear(bg_color)?;

        /* // Part of the tutorial but I mostly just hate it
        self.title.execute(|image| {
            let rect = &image.area().with_center((window.screen_size().x as i32 / 2, 40));
            let img = Img(&image);
            window.draw(rect, img);
            Ok(())
        })?;
        */

        // TODO: put these in an attributions page
        /*
        self.mononoki_font_info.execute(|image| {
            window.draw(&image.area().translate((20, window.screen_size().y as i32 - 80)), Img(&image));
            Ok(())
        })?;

        self.square_font_info.execute(|image| {
            window.draw(&image.area().translate((20, window.screen_size().y as i32 - 40)), Img(&image));
            Ok(())
        })?;
        */

        let offset_px = Vector::new(50, 50);

        render_map(offset_px, self, window)?;

        render_controls_image(self, window)?;

        Ok(())
    }
}

fn render_controls_image(game: &mut Game, window: &mut Window) -> QsResult<()> {
    if !game.controls_pane.show_controls_image {
        return Ok(());
    }

    let controls_image = &mut game.controls_pane.controls_image;

    controls_image.execute(|image| {
        let screen_size = window.screen_size();
        let image_size = image.area().size;

        let render_pos = Vector::new(screen_size.x - image_size.x - 30.0, 30.0);
        window.draw(&image.area().translate(render_pos), Img(&image));
        Ok(())
    })
}

fn render_map(offset_px: Vector, game: &mut Game, window: &mut Window) -> QsResult<()> {
    let tile_size_px = game.tile_size_px;

    let (tileset, map, camera, player) = (&mut game.tileset, &mut game.map, &game.camera, &game.player);

    let x_min = camera.x_min;
    let x_max = camera.x_max;

    let y_min = camera.y_min;
    let y_max = camera.y_max;

    tileset.execute(|tileset| {
        map.execute(|map| {
            for x in x_min..=x_max {
                for y in y_min..=y_max {
                    let square = map.get_square(x, y);
                    let square_char = square.to_char();

                    if let Some(image) = tileset.get(&square_char) {
                        let pos_px = Vector::new(x - x_min, y - y_min).times(tile_size_px) + offset_px;
                        let rect = Rectangle::new(pos_px, image.area().size());
                        window.draw(&rect, with_vis(Color::BLUE, square.visibility));
                        window.draw(&rect, Blended(&image, with_vis(Color::WHITE, square.visibility)));
                    }

                    // TODO: do something on unknown characters
                }
            }

            let image = tileset.get(&PLAYER_CHAR).expect("Player char had better be defined");

            let player_pos = player.tile_pos;
            let player_pos_px = Vector::new(player_pos.x - x_min, player_pos.y - y_min).times(tile_size_px) + offset_px;
            window.draw(&Rectangle::new(player_pos_px, image.area().size()), Blended(&image, Color::PURPLE));

            Ok(())
        })?;

        Ok(())
    })?;

    Ok(())
}
