use super::*;

use std::collections::HashMap;

use quicksilver::{
    geom::{Rectangle, Shape, Vector},
    graphics::{Color, Font, FontStyle, Image},
    lifecycle::{run, Asset, Settings, State, Window},
    load_file,
    prelude::*,
    Future,
};

use crate::maps::{Map, Square};

const PLAYER_CHAR: char = '@';

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct TilePos {
    x: i32,
    y: i32,
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

impl Player {
    fn try_move(&mut self, translation: TilePos, map: &Map) -> bool {
        let new_pos = TilePos {
            x: self.tile_pos.x + translation.x,
            y: self.tile_pos.y + translation.y,
        };
        let to_move_to = map.get_square(new_pos.x, new_pos.y);

        let can_move = match to_move_to {
            Square::Void => false,
            Square::Wall => false,
            Square::Open => false,

            Square::Floor => true,
            Square::HorizontalDoor => true,
            Square::VerticalDoor => true,
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

    title: Asset<Image>,
    mononoki_font_info: Asset<Image>,
    square_font_info: Asset<Image>,

    tileset: Asset<HashMap<char, Image>>,
    tile_size_px: Vector,
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

        let map: Asset<Map> = Asset::new(load_file("config/map_params.ron").and_then(|bytes| {
            let map_gen_params = ron::de::from_bytes(&bytes).expect("Should deserialize");
            let map: Map = Map::make_random(&map_gen_params);
            Ok(map)
        }));

        let player = Player {
            // TODO: make this square guaranteed to not be empty
            tile_pos: TilePos { x: 10, y: 10 },
        };

        let camera = CameraInfo {
            x_min: 0,
            x_max: 20,

            y_min: 0,
            y_max: 20,
        };

        Ok(Game {
            title,
            mononoki_font_info,
            square_font_info,

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

        if keyboard[Key::Escape] == Pressed {
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
                }
            }

            Ok(())
        })?;

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> QsResult<()> {
        window.clear(Color::WHITE)?;

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

        Ok(())
    }
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
            for x in x_min..(x_max + 1) {
                for y in y_min..(y_max + 1) {
                    let square_char = map.get_square(x, y).to_char();

                    if let Some(image) = tileset.get(&square_char) {
                        let pos_px = Vector::new(x - x_min, y - y_min).times(tile_size_px) + offset_px;
                        window.draw(&Rectangle::new(pos_px, image.area().size()), Blended(&image, Color::BLACK));
                    } else {
                        println!("ERROR: Unrecognized char: {}", square_char);
                    }
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
