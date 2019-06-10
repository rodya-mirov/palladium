#![allow(dead_code)] // TODO: delete this
#![allow(unused_imports)]

mod maps;
mod rng;
mod skills;
mod ui;

use std::fs::File;
use std::io::Read;

use quicksilver::{
    geom::Vector,
    lifecycle::{run, Settings, State, Window},
    Result as QsResult,
};

fn main() {
    let settings = Settings {
        scale: quicksilver::graphics::ImageScaleStrategy::Blur,
        ..Default::default()
    };
    run::<ui::Game>("Palladium", Vector::new(800, 600), settings);
}
