#![allow(clippy::collapsible_if)]
#![allow(clippy::needless_range_loop)]
// #![deny(clippy::print_stdout)] // TODO: do we need this

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

#[macro_use]
mod timer;

mod components;
mod game_state;
mod numerics;
mod resources;
mod rng;
mod skills;
mod systems;
mod world;

const CLEAR: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
    a: 0.0,
};

fn main() {
    let settings = Settings {
        scale: quicksilver::graphics::ImageScaleStrategy::Blur,
        ..Default::default()
    };

    run::<game_state::MainState>("Palladium", Vector::new(800, 600), settings);
}
