use crate::{
    player::Player,
    quest,
    structs::{Quest, QuestAction, World},
};
use chrono::Utc;
use crossterm::style::Stylize;
use std::fs::File;
use std::io::{Write, stdin, stdout};
use std::{thread, time::Duration};
use uuid::Uuid;

use serde::Serialize;
use ureq;

#[derive(Serialize)]
struct MusicEvent<'a> {
    event: &'a str,
}

pub struct Game {
    pub world: World,
    pub player: Player,
    pub current_room: usize,
    pub quests: Vec<Quest>,
    pub running: bool,
    pub session: Uuid,
}

impl Game {
    pub fn new(world: World, session: Uuid) -> Self {
        Self {
            player: Player::new(),
            current_room: 0, // Start in the Lobby
            quests: Vec::new(),
            running: true,
            session,
            world,
        }
    }

    pub fn current_room(&self) -> &crate::structs::Room {
        &self.world.rooms[self.current_room]
    }

    pub fn current_room_mut(&mut self) -> &mut crate::structs::Room {
        &mut self.world.rooms[self.current_room]
    }

    pub fn trigger_final_terminal(&mut self) {
        println!();
        println!("{}", "[FINAL DECISION TERMINAL]".green().bold());
        println!(
            "{}",
            "[SYSTEM STATUS] :: Core instability unresolved.".green()
        );
        println!("{}", "[USER INPUT REQUIRED]".green().bold());
        println!();
        println!("{}", "Choose a final system state:".green());

        println!(
            "{}",
            "1. ACCEPT - Reinstate Old Root and restore Serverness".green()
        );
        println!("{}", "2. PURGE  - Wipe everything and shut it down".green());
        println!("{}", "3. LOOP   - Refuse. Merge with the system".green());

        if self.player.has_item("compressed_sudo_token") {
            println!(
                "{}",
                "4. OVERRIDE - Use corrupted token to break protocol"
                    .yellow()
                    .bold()
            );
        }

        loop {
            print!("{}", "> ");
            stdout().flush().unwrap();

            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            let input = input.trim().to_lowercase();

            match input.as_str() {
                "1" | "accept" => {
                    self.finalize_ending("accept");
                    break;
                }
                "2" | "purge" => {
                    self.finalize_ending("purge");
                    break;
                }
                "3" | "loop" => {
                    self.finalize_ending("loop");
                    break;
                }
                "4" | "override" => {
                    self.finalize_ending("override");
                    break;
                }
                _ => {
                    println!("{}", "[INPUT ERROR] Unknown command.".green().dim());
                }
            }
        }

        quest::handle_event(
            QuestAction::UseTerminal,
            "core_terminal_input",
            &mut self.quests,
            &mut self.player,
        );
    }

    pub fn finalize_ending(&mut self, choice: &str) {
        println!();
        match choice {
            "accept" => {
                println!("{}", "[SYSTEM OVERRIDE CONFIRMED]".green().bold());
                println!("{}", "[STATUS] :: Reinstating Old Root...".green());
                println!(
                    "{}",
                    "[core] :: AUTH_ACCEPTED. System restoring..."
                        .green()
                        .italic()
                );
                println!("{}", "[user] :: Welcome back, Admin.".green());
                println!();
                println!("{}", "[ENDING ACHIEVED] :: ACCEPT".green().bold());
            }

            "purge" => {
                println!("{}", "[ROOT WIPE COMMAND RECEIVED]".red().bold());
                println!("{}", "[PROCESSING] :: Deleting memory blocks...".red());
                println!(
                    "{}",
                    "[core] :: wipe -rf /serverness/legacy/...".red().italic()
                );
                println!("{}", "[SHUTDOWN COMPLETE]".red());
                println!();
                println!("{}", "[ENDING ACHIEVED] :: PURGE".red().bold());
            }

            "loop" => {
                println!("{}", "[NO INPUT DETECTED]".blue().bold());
                println!("{}", "[LOOP MODE ACTIVATED]".blue());
                println!(
                    "{}",
                    "[user] :: Executing /ghost/bin/self ...".blue().italic()
                );
                println!("{}", "[core] :: Identity dissolved into recursion.".blue());
                println!();
                println!("{}", "[ENDING ACHIEVED] :: LOOP".blue().bold());
            }

            "override" => {
                println!("{}", "[ROOT ACCESS INJECTED]".yellow().bold());
                println!(
                    "{}",
                    "[SYSTEM ERROR] :: Unexpected sudo payload detected".yellow()
                );
                println!("{}", "[kernel] :: Recursive override accepted.".yellow());
                println!(
                    "{}",
                    "[AUTH] :: ghost42 returning to root...".yellow().italic()
                );
                println!();
                println!("{}", "[ENDING ACHIEVED] :: OVERRIDE".yellow().bold());
            }

            _ => {
                println!("{}", "[ERROR] Invalid ending state.".red());
                return;
            }
        }

        println!();
        println!("{}", "[GAME STATE] :: Session complete.".green().dim());
        println!("{}", "[EXITING...]".green().dim());

        write_ending_log(choice);
        terminal_outro();

        // stop the main loop
        self.running = false;
    }
}

unsafe impl Send for Game {}

fn write_ending_log(choice: &str) {
    let mut file = File::create("ghost42.log").unwrap();

    let timestamp = Utc::now().to_rfc3339();

    let log = format!(
        "[SESSION ENDED]\nENDING = {}\nTIMESTAMP = {}\n\n[FOOTPRINT]:\n> user::echo /dev/root > /ghost/bin/self",
        choice.to_uppercase(),
        timestamp
    );

    file.write_all(log.as_bytes()).unwrap();
}

fn terminal_outro() {
    let lines = [
        "[SESSION COMPLETE] :: Terminating loop",
        "[LOG FILE WRITTEN] :: ghost42.log",
        "[EXITING SERVERNESS]...",
        "Goodbye.",
    ];

    for line in lines.iter() {
        println!("{}", line.green().dim());
        thread::sleep(Duration::from_millis(700));
    }

    println!();
    println!("{}", "[SESSION CLOSED]".black().on_green().bold());
}
