#![allow(clippy::needless_range_loop)] // I'll decide which one is more readable
#![allow(clippy::collapsible_if)] // I like it this way sometimes, for symmetry
#![deny(clippy::print_stdout)] // For real though, print! will crash the wasm build
#![allow(dead_code)] // TODO: delete this

mod maps;
mod numerics;
mod rng;
mod skills;
mod state;
mod ui;

use quicksilver::{
    geom::Vector,
    lifecycle::{run, Settings},
    Result as QsResult,
};

fn main() {
    let settings = Settings {
        scale: quicksilver::graphics::ImageScaleStrategy::Blur,
        ..Default::default()
    };
    run::<state::Game>("Palladium", Vector::new(800, 600), settings);
}
