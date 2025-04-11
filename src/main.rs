mod battle;
mod commands;
mod game;
mod player;
mod quest;
mod structs;
mod world;

use crossterm::{
    execute,
    style::Stylize,
    style::{Color, SetBackgroundColor},
    terminal::{Clear, ClearType},
};
use figlet_rs::FIGfont;
use reedline::{Reedline, Signal};
use std::{cell::RefCell, rc::Rc};
use std::{io::stdout, path::PathBuf};
mod ui;
use std::thread::sleep;
use std::time::Duration;
use ui::prompt::GamePrompt;

fn main() {
    let mut line_editor = Reedline::create();

    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("world.json");

    let world = world::load_world(path.to_str().unwrap());
    let game = Rc::new(RefCell::new(game::Game::new(world)));

    let prompt = GamePrompt {
        game: Rc::clone(&game),
    };

    clear_screen();
    boot_sequence();
    clear_screen();
    prologue();
    clear_screen();

    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("Serverness");
    if let Some(ref banner) = figure {
        println!("{}", banner.to_string().green());
    }

    println!(
        "{}",
        "[TERMINAL QUEST INTERFACE INITIALIZED]".green().bold()
    );
    println!(
        "{}",
        "type 'help' for a list of valid commands\n".green().dim()
    );

    // commands::Command::Look.handle(&mut game.borrow_mut());

    loop {
        match line_editor.read_line(&prompt) {
            Ok(Signal::Success(input)) => {
                let command = commands::Command::parse(&input);
                command.handle(&mut game.borrow_mut());

                if !game.borrow().player.is_alive() {
                    println!(
                        "\n{} {}",
                        "[SESSION ENDED]".red().bold(),
                        format!("FINAL SCORE: {}", game.borrow().player.points)
                            .green()
                            .bold()
                    );

                    break;
                }
            }
            Ok(Signal::CtrlC | Signal::CtrlD) => {
                println!("{}", "\n[LINK TERMINATED] User exited.".green().dim());

                break;
            }
            _ => continue,
        }
    }
}

fn clear_screen() {
    println!("{}", "\x1B[?5h"); // Inverse video mode ON (flicker)
    std::thread::sleep(std::time::Duration::from_millis(80));
    println!("{}", "\x1B[?5l"); // Inverse video mode OFF
    let _ = execute!(stdout(), Clear(ClearType::All));
}

fn boot_sequence() {
    let delay = Duration::from_millis(50);

    println!("{}", "[BOOT SEQUENCE INITIATED]".green().bold());
    sleep(delay);

    println!("{}", ">> Detecting memory modules... [OK]".green());
    sleep(delay);

    println!("{}", ">> Initializing packet bus... [OK]".green());
    sleep(delay);

    println!("{}", ">> Loading quest protocols... [OK]".green());
    sleep(delay);

    println!("{}", ">> Verifying artifact index... [OK]".green());
    sleep(delay);

    println!("{}", ">> Syncing lore database... [OK]".green());
    sleep(delay);

    println!("{}", ">> Establishing neural link... [OK]".green());
    sleep(delay);

    println!("{}", ">> Preparing command shell... [OK]".green());
    sleep(delay);

    println!("{}", "[SYSTEM READY]".green().bold());
    sleep(Duration::from_millis(500));
}

fn prologue() {
    let lines = [
        "[RELAY] Secure link established: /serverness/root",
        "",
        "",
        "> SYSTEM REPORT: Critical subsystem integrity = 14%",
        "> Last Root Commit: 1823 days ago",
        "> Primary Core: degraded",
        "> Dev Loop: unresolved",
        "> Exit Traffic: rerouted to /dev/null",
        "",
        "You are a stranded maintenance process — booted into a dead network.",
        "Everything went silent during the Great Shutdown. No pings. No logs. No ACKs.",
        "",
        "The CoreRouter floats in sleep mode.",
        "Daemon processes flicker, half-dead and half-sane.",
        "Legacy NPCs speak in fragments.",
        "The system is asking:",
        "",
        "> Should I reboot?",
        "> Or decay in silence?",
        "",
        "Only you can trace the lost protocols, recompile trust, and reactivate the forgotten Core.",
        "",
        "type 'look' to begin.",
    ];

    for line in lines {
        println!("{}", line.green());
        sleep(Duration::from_millis(45));
    }
    println!();
    println!("{}", "[PRESS ENTER TO CONTINUE]".green().bold());
    let _ = std::io::stdin().read_line(&mut String::new());
}
