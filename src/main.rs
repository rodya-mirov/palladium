mod skills;

use std::fs::File;
use std::io::Read;

fn main() {
    let mut file_contents = String::new();
    File::open("resources/skills.ron")
        .expect("File should exist")
        .read_to_string(&mut file_contents)
        .expect("File should read");

    let tree = skills::SkillTree::from_ron(&file_contents).expect("Should parse");
    println!("Skills: {:?}", tree);
}
