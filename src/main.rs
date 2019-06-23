#![allow(clippy::collapsible_if)]
#![allow(clippy::needless_range_loop)]

#[macro_use]
extern crate specs_derive;

use specs::Component;

use quicksilver::{
    geom::Vector,
    lifecycle::{run, Asset, Settings},
    Result as QsResult,
};

mod components;
mod game_state;
mod numerics;
mod rng;
mod skills;
mod systems;
mod world;

fn main() {
    let settings = Settings {
        scale: quicksilver::graphics::ImageScaleStrategy::Blur,
        ..Default::default()
    };
    run::<game_state::MainState>("Palladium", Vector::new(800, 600), settings);
}
