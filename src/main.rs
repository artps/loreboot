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
    terminal::{Clear, ClearType},
};
use figlet_rs::FIGfont;
use reedline::{Reedline, Signal};
use std::io::stdout;
use std::{cell::RefCell, rc::Rc};
mod ui;
use std::thread::sleep;
use std::time::Duration;
use ui::prompt::GamePrompt;

fn main() {
    let mut line_editor = Reedline::create();

    let world = world::load_world("world.json");
    let game = Rc::new(RefCell::new(game::Game::new(world)));

    let prompt = GamePrompt {
        game: Rc::clone(&game),
    };

    clear_screen();
    boot_sequence();
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

    commands::Command::Look.handle(&mut game.borrow_mut());

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
