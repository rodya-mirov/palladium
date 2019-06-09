mod maps;
mod skills;
mod rng;

use std::fs::File;
use std::io::Read;

fn run_skill_stuff() {
    let mut file_contents = String::new();
    File::open("resources/skills.ron")
        .expect("File should exist")
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
    let mut map_param_str = String::new();
    File::open("resources/map_params.ron")
        .expect("File should exist")
        .read_to_string(&mut map_param_str)
        .expect("Should be able to read");

    let map_params: maps::MapGenerationParams = ron::de::from_str(&map_param_str).expect("Should parse");

    let map = maps::Map::make_random(&map_params);

    map.draw();
}

fn main() {
    run_skill_stuff();
    run_map_stuff();
}
