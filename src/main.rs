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

fn get_map_gen_config() -> maps::MapGenerationParams {
    let mut map_param_str = String::new();
    File::open("static/config/map_params.ron")
        .expect("Map config file should exist")
        .read_to_string(&mut map_param_str)
        .expect("Should be able to read");

    ron::de::from_str(&map_param_str).expect("Should parse")
}

fn run_skill_stuff() {
    let mut file_contents = String::new();
    File::open("static/config/skills.ron")
        .expect("Skills config file should exist")
        .read_to_string(&mut file_contents)
        .expect("File should read");

    let mut tree = skills::SkillTree::from_ron(&file_contents).expect("Should parse");

    tree.gain_experience(skills::ExpGain {
        skill_name: "Dexterity",
        exp: 31,
    })
    .expect("Should be fine");

    println!("Skills: {:?}", tree);

    println!(
        "Dexterity has {} experience",
        tree.check_experience("Dexterity").expect("Should be defined")
    );
}

fn run_map_stuff() {
    let map_params = get_map_gen_config();

    let map = maps::Map::make_random(&map_params);

    map.draw();
}

fn main() {
    get_map_gen_config();

    let settings = Settings {
        scale: quicksilver::graphics::ImageScaleStrategy::Blur,
        ..Default::default()
    };
    run::<ui::Game>("Palladium", Vector::new(800, 600), settings);
}
