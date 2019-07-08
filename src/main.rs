#![allow(clippy::collapsible_if)]
#![allow(clippy::needless_range_loop)]
// #![deny(clippy::print_stdout)] // TODO: do we need this

#[macro_use]
extern crate specs;

// This is a clippy bug; this trait import is used, in a submodule
#[allow(unused_imports)]
use specs::{prelude::*, saveload::MarkedBuilder};

use serde::{Deserialize, Serialize};

use quicksilver::{
    geom::*,
    graphics::{Background::*, Color},
    lifecycle::{run, Asset, Settings},
    Result as QsResult,
};

#[macro_use]
mod timer;

#[macro_use]
mod components;

mod constants;
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
