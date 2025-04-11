// use crate::battle::{display_battle_ui, enemy_turn, player_turn};
use crate::player::Player;
use crate::structs::Enemy;
use crate::structs::World;
use crossterm::style::Stylize;
use rand::Rng;
// use rand::thread_rng;

pub fn start_battle(player: &mut Player, enemy: &Enemy, world: &World) -> bool {
    println!("{}", "[BREACH ATTEMPT INITIALIZED]".green().bold());
    println!(
        "{} {}",
        "> TARGET".green(),
        enemy.name.to_uppercase().green().bold()
    );

    let mut enemy_hp = enemy.hp;
    let mut rng = rand::rng();

    loop {
        display_battle_ui(player.hp, enemy_hp, &enemy.name);

        // === Player Turn ===
        if let Some(outcome) = player_turn(player, &mut enemy_hp, &enemy.name, &mut rng, world) {
            return outcome; // early return if player wins or flees
        }

        if enemy_hp <= 0 {
            println!("{}", "[INTRUSION SUCCESSFUL]".green().bold());

            return true;
        }

        // === Enemy Turn ===
        let outcome = enemy_turn(player, enemy, &mut rng);
        if !outcome {
            println!();
            println!("{}", "[!!! SYSTEM FAILURE DETECTED]".red().bold());
            println!(
                "{} {}",
                "> HOSTILE OVERRIDE".red(),
                enemy.name.to_uppercase().red().bold()
            );
            println!("{}", "[INTRUSION REJECTED]".on_green().black());
            println!(
                "{}",
                "CORE DUMP FAILED — STACK TRACE UNSTABLE".dim().green()
            );
            println!("{}", "[SYSTEM HALTED]".bold().green());
            return false;
        }
    }
}

pub fn display_battle_ui(player_hp: i32, enemy_hp: i32, enemy_name: &str) {
    println!();
    println!("{}", "[COMBAT TRACE ACTIVE]".green().bold());
    println!(
        "{} {}",
        "> HOSTILE".green(),
        enemy_name.to_uppercase().green()
    );
    println!(
        "{} {}",
        "> ENEMY HP".green(),
        enemy_hp.max(0).to_string().green()
    );
    println!(
        "{} {}",
        "> YOUR HP".green(),
        player_hp.max(0).to_string().green()
    );

    println!();
    println!("{}", "[ACTIONS AVAILABLE]".green().bold());
    println!("{}", "- attack     - block".green());
    println!("{}", "- use <item> - flee".green());
}

pub fn player_turn<R: Rng>(
    player: &mut Player,
    enemy_hp: &mut i32,
    enemy_name: &str,
    rng: &mut R,
    world: &World,
) -> Option<bool> {
    use std::io::{Write, stdin, stdout};

    print!("> ");
    stdout().flush().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input = input.trim().to_lowercase();

    match input.as_str() {
        "attack" => {
            let hit_chance = 0.9 - player.agility as f64 * 0.01 + (player.luck as f64 * 0.005);

            if rng.random_bool(hit_chance) {
                let base = rng.random_range(3..8) + player.strength / 2;
                let crit_chance =
                    0.1 + (player.luck as f64 * 0.01) + (player.intelligence as f64 * 0.005);
                let crit = rng.random_bool(crit_chance);
                let damage = if crit { base * 2 } else { base };

                *enemy_hp = enemy_hp.saturating_sub(damage);

                println!(
                    "{} {}{}",
                    "> STRIKE SUCCESS".green(),
                    damage.to_string().green().bold(),
                    if crit { " (CRIT)" } else { "" }
                );
            } else {
                println!("{}", "[MISS] You failed to land a hit.".green().dim());
            }
        }

        "block" => {
            println!("{}", "[DEFENSIVE STANCE ENABLED]".green().bold());

            player.status.blocking = true;
        }

        "flee" => {
            let flee_chance = 0.5 + (player.luck as f64 * 0.02);

            if rng.random_bool(flee_chance) {
                println!("{}", "[EVADE SUCCESSFUL] Exit granted.".green().bold());

                return Some(false); // flee = survive but fail combat
            } else {
                println!("{}", "[EVADE FAILED] Lockdown enforced.".on_green().black());
            }
        }

        other if other.starts_with("use ") => {
            let item_name = other.strip_prefix("use ").unwrap().trim();

            if player.use_item(item_name, world) {
                println!("{}", "[EMERGENCY PATCH APPLIED]".green().bold());
                println!(
                    "{} {}",
                    "> MODULE".green(),
                    item_name.to_uppercase().green().bold()
                );
            } else {
                println!(
                    "{} {}",
                    "[INVENTORY MISMATCH]".green(),
                    item_name.to_uppercase().dim().green()
                );
            }
        }

        _ => {
            println!("{}", "[UNKNOWN INPUT]".green().dim());
        }
    }

    None
}

pub fn enemy_turn<R: Rng>(player: &mut Player, enemy: &Enemy, rng: &mut R) -> bool {
    println!(
        "{} {}",
        "[HOSTILE SIGNAL]".green(),
        format!("{} executing routine...", enemy.name).green()
    );

    // Basic enemy AI: mostly attacks, sometimes taunts
    if rng.random_bool(0.8) {
        let mut damage = rng.random_range(2..enemy.strength + 4);
        if player.status.blocking {
            println!("{}", "[BLOCK ACTIVE] Damage reduced.".green().dim());

            damage /= 2;
        }

        player.take_damage(damage);

        println!(
            "{} {}",
            "[DAMAGE RECEIVED]".green(),
            format!("{} HP", damage).green().bold()
        );
    } else {
        println!(
            "{}",
            "[PROCESS HANG] Hostile command dropped.".green().dim()
        );
    }

    // Reset block status
    player.status.blocking = false;

    player.is_alive()
}
