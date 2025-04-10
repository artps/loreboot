use crate::structs::World;
use std::{collections::HashMap, fs, path::PathBuf};

pub fn load_world(path: &str) -> World {
    let data = fs::read_to_string(path).expect("Failed to read world.json");
    let mut world: World = serde_json::from_str(&data).expect("Invalid world.json format");

    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("log_pools.json");

    let log_data = fs::read_to_string(path).expect("Missing log_pools.json");
    let log_pool: HashMap<String, Vec<String>> =
        serde_json::from_str(&log_data).expect("Invalid log_pools.json format");

    inject_dynamic_logs(&mut world, &log_pool);

    world
}

fn inject_dynamic_logs(world: &mut World, log_pool: &HashMap<String, Vec<String>>) {
    for room in &mut world.rooms {
        for lore in &mut room.lore {
            if lore.generated.unwrap_or(false) {
                let role = room.role.as_deref().unwrap_or("random");
                let pool = log_pool.get(role).or_else(|| log_pool.get("random"));

                if let Some(lines) = pool {
                    use rand::seq::IteratorRandom;
                    let mut rng = rand::rng();
                    lore.logs = lines
                        .into_iter()
                        .choose_multiple(&mut rng, 4)
                        .into_iter()
                        .cloned()
                        .collect();
                }
            }
        }
    }
}
