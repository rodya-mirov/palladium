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

use crate::maps::Map;

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
    fn translate(&mut self, dx: i32, dy: i32) {
        self.x_min += dx;
        self.x_max += dx;
        self.y_min += dy;
        self.y_max += dy;
    }
}

pub struct Game {
    map: Asset<Map>,

    camera: CameraInfo,

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

        let camera = CameraInfo {
            x_min: -3,
            x_max: 18,

            y_min: -3,
            y_max: 18,
        };

        Ok(Game {
            title,
            mononoki_font_info,
            square_font_info,

            camera,
            map,

            tile_size_px,
            tileset,
        })
    }

    fn update(&mut self, window: &mut Window) -> QsResult<()> {
        use quicksilver::input::ButtonState::*;

        let keyboard = window.keyboard();

        if keyboard[Key::Left] == Pressed {
            self.camera.translate(-1, 0);
        } else if keyboard[Key::Up] == Pressed {
            self.camera.translate(0, -1);
        } else if keyboard[Key::Down] == Pressed {
            self.camera.translate(0, 1);
        } else if keyboard[Key::Right] == Pressed {
            self.camera.translate(1, 0);
        }
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

    let (tileset, map, camera) = (&mut game.tileset, &mut game.map, &game.camera);

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

            Ok(())
        })
    })?;

    Ok(())
}
