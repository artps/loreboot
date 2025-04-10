use crate::structs::World;
use std::fs;

pub fn load_world(path: &str) -> World {
    let data = fs::read_to_string(path).expect("Failed to read world.json");
    let world: World = serde_json::from_str(&data).expect("Invalid world.json format");
    world
}
