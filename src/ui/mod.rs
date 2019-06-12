use super::*;

use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::Color,
    lifecycle::Window,
    prelude::*,
};

use crate::maps::VisibilityType;
use crate::state::Game;

const PLAYER_CHAR: char = '@';

fn with_vis(color: Color, visibility: VisibilityType) -> Color {
    let alpha = match visibility {
        VisibilityType::NotSeen => 0.0,
        VisibilityType::Remembered => 0.3,
        VisibilityType::CurrentlyVisible => 1.0,
    };

    color.with_alpha(alpha)
}

pub fn render_game(game: &mut Game, window: &mut Window) -> QsResult<()> {
    let bg_color = Color::from_hex("556887");
    window.clear(bg_color)?;

    let offset_px = Vector::new(50, 50);

    render_map(offset_px, game, window)?;

    render_controls_image(game, window)?;

    Ok(())
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
