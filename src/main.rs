#![allow(clippy::collapsible_if)]
#![allow(clippy::needless_range_loop)]

#[macro_use]
extern crate specs_derive;

#[macro_use]
extern crate shred_derive;

use specs::prelude::*;

use quicksilver::{
    lifecycle::{run, Asset, Settings},
    prelude::*,
    Result as QsResult,
};

mod components;
mod game_state;
mod numerics;
mod resources;
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
