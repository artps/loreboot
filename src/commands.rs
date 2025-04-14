use crate::{
    battle,
    game::Game,
    music, quest,
    structs::{IdRef, QuestAction},
};
use crossterm::style::Stylize;

pub enum Command {
    Look,
    Go(String),
    Inspect(String),
    Take(String),
    Examine(String),
    Scan,
    Talk(String),
    Attack(String),
    Use(String),
    Inventory,
    Status,
    Quests,
    Help,
    Unknown(String),
}

impl Command {
    pub fn parse(input: &str) -> Self {
        let words: Vec<&str> = input.trim().split_whitespace().collect();
        if words.is_empty() {
            return Command::Unknown("".into());
        }

        match words[0].to_lowercase().as_str() {
            "look" => Command::Look,
            "go" if words.len() > 1 => Command::Go(words[1].to_string()),
            "inspect" if words.len() > 1 => Command::Inspect(words[1..].join(" ")),
            "take" if words.len() > 1 => Command::Take(words[1..].join(" ")),
            "examine" if words.len() > 1 => Command::Examine(words[1..].join(" ")),
            "talk" if words.len() > 1 => Command::Talk(words[1..].join(" ")),
            "attack" if words.len() > 1 => Command::Attack(words[1..].join(" ")),
            "use" if words.len() > 1 => Command::Use(words[1..].join(" ")),
            "inventory" => Command::Inventory,
            "status" => Command::Status,
            "quests" => Command::Quests,
            "help" => Command::Help,
            "scan" => Command::Scan,
            _ => Command::Unknown(words.join(" ")),
        }
    }

    pub fn handle(&self, game: &mut Game) {
        let session = game.session.clone();

        match self {
            Command::Look => {
                let room = game.current_room();

                music::send_event(session.clone(), &format!("room:{}", room.id));

                // Room Header
                println!("{}", "[SCAN COMPLETE]".green().bold());
                println!(
                    "{}",
                    format!("[{}]", room.name.to_uppercase()).magenta().bold()
                );
                println!("{}", room.description.clone().green());

                // Show items
                if !room.items.is_empty() {
                    println!("\n{}", ":: ARTIFACTS LOCATED ::".green().bold());

                    for item_ref in &room.items {
                        if let Some(item) = game.world.items.iter().find(|i| i.id == item_ref.id) {
                            println!("  - {}", item.id.to_uppercase().green());
                        }
                    }
                }

                // Show enemies
                if !room.enemies.is_empty() {
                    println!("\n{}", ":: HOSTILES DETECTED ::".red().bold());

                    for enemy_ref in &room.enemies {
                        if let Some(enemy) =
                            game.world.enemies.iter().find(|e| e.id == enemy_ref.id)
                        {
                            println!("  - {}", enemy.id.to_uppercase().red().bold());
                        }
                    }
                }

                // Show NPCs
                if !room.npcs.is_empty() {
                    println!("\n{}", ":: HUMAN INTERFACES ::".cyan().bold());

                    for npc_ref in &room.npcs {
                        if let Some(npc) = game.world.npcs.iter().find(|n| n.id == npc_ref.id) {
                            println!("  - {}", npc.id.to_uppercase().cyan());
                        }
                    }
                }

                // Show exits
                if !room.exits.is_empty() {
                    println!("\n{}", ":: NETWORK LINKS ::".blue().bold());

                    for exit in &room.exits {
                        if let Some(dest) = game.world.rooms.iter().find(|r| r.id == exit.to) {
                            println!(
                                "  > {} {} {}",
                                "[".green(),
                                exit.direction.to_uppercase().blue().bold(),
                                format!("→ {}]", dest.name.to_uppercase()).green()
                            );
                        }
                    }
                }

                let visible_lore: Vec<_> = room.lore.iter().filter(|l| !l.hidden).collect();
                if !visible_lore.is_empty() {
                    println!("\n{}", ":: SYSTEM REMNANTS ::".yellow().bold());
                    for l in visible_lore {
                        println!(" - {}", l.name.clone().yellow());
                    }
                }
            }

            Command::Go(direction) => {
                let exits = &game.current_room().exits;
                if let Some(exit) = exits
                    .iter()
                    .find(|e| e.direction.to_lowercase() == direction.to_lowercase())
                {
                    // Check for locked exit
                    if exit.locked.unwrap_or(false) {
                        let mut locked = false;

                        // Lock by item
                        if let Some(ref item_id) = exit.required_item_id {
                            let has_item = game.player.inventory.iter().any(|i| &i.id == item_id);
                            if !has_item {
                                locked = true;
                            }
                        }

                        // Lock by quest step
                        if let (Some(quest_id), Some(step_id)) =
                            (&exit.required_quest_id, &exit.required_step)
                        {
                            let found = game.quests.iter().any(|q| {
                                q.id == *quest_id
                                    && q.steps.iter().any(|s| s.id == *step_id && s.completed)
                            });
                            if !found {
                                locked = true;
                            }
                        }

                        if locked {
                            let msg = exit.locked_msg.as_deref().unwrap_or("The way is blocked.");
                            println!(
                                "{} {}",
                                "[ACCESS DENIED]".on_green().black(),
                                msg.green().italic()
                            );
                            return;
                        }
                    }

                    game.current_room = exit.to;
                    let room = game.current_room();
                    music::send_event(session.clone(), &format!("room:{}", room.id));

                    println!("{}", "[TRACE ROUTE]".bold().green());
                    println!(
                        "{} {}",
                        ">> HOP →".green(),
                        room.name.to_uppercase().green().bold()
                    );
                    println!("{}", room.description.clone().green());

                    println!("{}", "[LINK ESTABLISHED]".green());
                    std::thread::sleep(std::time::Duration::from_millis(300));

                    game.player.add_points(room.points);
                } else {
                    println!("🚧 No exit in that direction.");
                }
            }

            Command::Inspect(direction) => {
                let exits = &game.current_room().exits;
                if let Some(exit) = exits
                    .iter()
                    .find(|e| e.direction.eq_ignore_ascii_case(direction))
                {
                    let mut locked = false;
                    let mut reasons = vec![];

                    if let Some(ref item_id) = exit.required_item_id {
                        if !game
                            .player
                            .inventory
                            .iter()
                            .any(|i| i.id.to_lowercase() == item_id.to_lowercase())
                        {
                            let label = game
                                .world
                                .items
                                .iter()
                                .find(|it| &it.id == item_id)
                                .map(|it| it.name.clone())
                                .unwrap_or_else(|| item_id.clone());
                            reasons.push(format!("MISSING ITEM: {}", label.to_uppercase()));
                            locked = true;
                        }
                    }

                    if let (Some(qid), Some(step_id)) =
                        (&exit.required_quest_id, &exit.required_step)
                    {
                        let found = game.quests.iter().any(|q| {
                            q.id == *qid && q.steps.iter().any(|s| s.id == *step_id && s.completed)
                        });

                        if !found {
                            reasons
                                .push(format!("QUEST STEP INCOMPLETE: {}", step_id.to_uppercase()));
                            locked = true;
                        }
                    }

                    let dest = game
                        .world
                        .rooms
                        .iter()
                        .find(|r| r.id == exit.to)
                        .map(|r| r.name.to_uppercase())
                        .unwrap_or("UNKNOWN".into());

                    println!();
                    println!(
                        "{}",
                        format!("[ACCESS GATE: {} → {}]", direction.to_uppercase(), dest)
                            .green()
                            .bold()
                    );

                    if locked {
                        println!("  :: STATUS     → {}", "LOCKED".red());
                        for reason in reasons {
                            println!("  :: REASON     → {}", reason);
                        }

                        if let Some(hint) = &exit.unlock_hint {
                            println!("  :: HINT       → {}", hint.to_uppercase().dim());
                        }
                    } else {
                        println!("  :: STATUS     → {}", "ACCESSIBLE".green());
                    }
                } else {
                    println!(
                        "{}",
                        format!(">> No exit in direction '{}'", direction).dim()
                    );
                }
            }

            Command::Take(item_id) => {
                let room = game.current_room_mut();
                if let Some(pos) = room
                    .items
                    .iter()
                    .position(|i| i.id.to_lowercase() == *item_id.to_lowercase())
                {
                    let item = room.items.remove(pos);

                    if let Some(def) = game
                        .world
                        .items
                        .iter()
                        .find(|it| it.id.to_lowercase() == item.id.to_lowercase())
                    {
                        game.player.add_item(&item.id, &game.world); // <-- applies trait_mods too

                        game.player.add_points(def.points);

                        println!("{}", "[EXTRACTING ITEM]".green().bold());
                        std::thread::sleep(std::time::Duration::from_millis(200));

                        println!(
                            "{} {}",
                            "[WRITE > MEMORY SEGMENT]".green(),
                            def.name.to_uppercase().bold().green()
                        );

                        println!(
                            "{} +{}",
                            "[POINT INJECTION]".green(),
                            def.points.to_string().green()
                        );

                        quest::handle_event(
                            QuestAction::CollectItem,
                            &item.id,
                            &mut game.quests,
                            &mut game.player,
                        );
                    }
                } else {
                    println!("{}", "[ITEM NOT FOUND]".red().bold());
                }
            }

            Command::Examine(target) => {
                // First check inventory
                if game.player.has_item(&target) {
                    if let Some(item) = game.world.items.iter().find(|i| {
                        i.id == *target || i.name.to_lowercase().contains(&target.to_lowercase())
                    }) {
                        println!("{}", "[ITEM DUMP]".bold().green());
                        println!(
                            "{}",
                            format!("[{}]", item.name.to_uppercase()).green().bold()
                        );
                        println!("{}", item.description.clone().green().italic());
                    }
                    return;
                }

                let (effect, lore_id) = {
                    let room = game.current_room_mut();
                    if let Some(obj) = room.lore.iter().find(|l| {
                        !l.hidden
                            && (l.id == *target
                                || l.name.to_lowercase().contains(&target.to_lowercase()))
                    }) {
                        println!("{}", "[MEMORY TRACE DETECTED]".green().bold());
                        println!("{}", obj.description.clone().green());

                        for log in &obj.logs {
                            println!("{}", format!("> {}", log).green().dim().italic());
                        }

                        (obj.trigger_effect.clone(), Some(obj.id.clone()))
                    } else {
                        println!(
                            "{}",
                            "> scan.lore: void response :: 0x000".italic().dim().green()
                        );
                        (None, None)
                    }
                };

                if let Some(effect) = effect {
                    handle_trigger_effect(&effect, game);
                }

                if let Some(id) = lore_id {
                    quest::handle_event(
                        QuestAction::ReadLore,
                        &id,
                        &mut game.quests,
                        &mut game.player,
                    );
                }
            }

            Command::Scan => {
                let hidden_lore: Vec<_> = game
                    .current_room()
                    .lore
                    .iter()
                    .filter(|obj| obj.hidden)
                    .cloned()
                    .collect();

                println!("{}", "[SUBNET SCAN INITIALIZED]".green().bold());

                for obj in hidden_lore {
                    println!(
                        "{} {}",
                        "[TRACE FOUND]".green(),
                        obj.name.to_uppercase().bold().green()
                    );

                    for log in &obj.logs {
                        println!("{}", format!("> {}", log).green().dim().italic());
                    }

                    if let Some(effect) = &obj.trigger_effect {
                        handle_trigger_effect(effect, game);
                    }

                    quest::handle_event(
                        QuestAction::ReadLore,
                        &obj.id,
                        &mut game.quests,
                        &mut game.player,
                    );
                }
            }

            Command::Inventory => {
                game.player.list_inventory(&game.world);
            }

            Command::Status => {
                let p = &game.player;

                println!("{}", "[CORE STATUS DUMP]".green().bold());
                println!("{:<18} :: {}", "> HP".green(), format!("{}", p.hp).green());
                println!(
                    "{:<18} :: {}",
                    "> POINTS".green(),
                    format!("{}", p.points).green()
                );

                println!();
                println!("{}", "[TRAIT MATRIX]".green().bold());

                macro_rules! stat {
                    ($label:expr, $value:expr) => {
                        println!(
                            "{:<18} :: {}",
                            format!("> {}", $label).green(),
                            format!("{}", $value).green()
                        );
                    };
                }

                stat!("INTELLIGENCE", p.intelligence);
                stat!("PERSISTENCE", p.persistence);
                stat!("AGILITY", p.agility);
                stat!("CHARISMA", p.charisma);
                stat!("LUCK", p.luck);
                stat!("WISDOM", p.wisdom);
                stat!("STRENGTH", p.strength);
            }

            Command::Quests => {
                use crossterm::style::Stylize;

                if game.quests.is_empty() {
                    println!("{}", "[QUEST LOG EMPTY]".green().dim());
                    return;
                }

                println!("{}", "[QUEST SYNC COMPLETE]".green().bold());

                for quest in &game.quests {
                    let all_done = quest.steps.iter().all(|s| s.completed);
                    let status = if all_done { "[X]" } else { "[ ]" };

                    println!(
                        "\n{} {}",
                        status.green(),
                        quest.name.to_uppercase().green().bold()
                    );

                    for step in &quest.steps {
                        let step_status = if step.completed { "[X]" } else { "[ ]" };
                        let line = format!("{} {}", step_status, step.description);

                        if step.completed {
                            println!("{}", format!("  {}", line).dim().green().italic());
                        } else {
                            println!("{}", format!("  {}", line).green());
                        }
                    }
                }
            }

            Command::Talk(npc_id) => {
                let room = game.current_room();

                let npc = room
                    .npcs
                    .iter()
                    .filter_map(|npc_ref| game.world.npcs.iter().find(|n| n.id == npc_ref.id))
                    .collect::<Vec<_>>();
                let npc = match_id_or_name(npc_id, &npc, |n| &n.id, |n| &n.name);

                if let Some(npc) = npc {
                    println!("{}", "[COMM CHANNEL OPENED]".green().bold());
                    println!(
                        "{} {}",
                        "> ID:".green(),
                        npc.name.to_uppercase().green().bold()
                    );
                    println!(
                        "{} {}",
                        "> MSG:".green(),
                        npc.dialogue.clone().green().italic()
                    );

                    game.player.add_points(npc.points);

                    if let Some(quest_id) = &npc.quest_to_give {
                        let already_has = game.quests.iter().any(|q| &q.id == quest_id);
                        if !already_has {
                            if let Some(quest) =
                                game.world.quests.iter().find(|q| &q.id == quest_id)
                            {
                                game.quests.push(quest.clone());
                                println!(
                                    "{} {}",
                                    "[MISSION LINK ESTABLISHED]".green(),
                                    quest.name.to_uppercase().green().bold()
                                );
                            }
                        }
                    }

                    quest::handle_event(
                        QuestAction::TalkNpc,
                        &npc.id,
                        &mut game.quests,
                        &mut game.player,
                    );
                } else {
                    println!(
                        "{} {}",
                        "[RECV ERROR]".on_green().black(),
                        format!("NO INTERFACE MATCHING '{}'", npc_id.to_uppercase()).green()
                    );
                }
            }

            Command::Attack(enemy_id) => {
                let room = game.current_room_mut();
                let room_id = &format!("room:{}", room.id.clone());
                // let session = game.session_mut();

                if let Some(pos) = room
                    .enemies
                    .iter()
                    .position(|e| e.id.to_lowercase() == *enemy_id.to_lowercase())
                {
                    let enemy_ref = room.enemies.remove(pos);
                    if let Some(enemy_def) =
                        game.world.enemies.iter().find(|e| e.id == enemy_ref.id)
                    {
                        music::send_event(session.clone(), &format!("battle:{}", enemy_def.id));

                        let won = battle::start_battle(&mut game.player, enemy_def, &game.world);
                        if won {
                            music::send_event(session.clone(), room_id);

                            game.player.add_points(enemy_def.points);
                            quest::handle_event(
                                QuestAction::DefeatEnemy,
                                &enemy_def.id,
                                &mut game.quests,
                                &mut game.player,
                            );
                        }
                    }
                } else {
                    println!("No such enemy here.");
                }
            }

            Command::Use(item_id) => {
                if game.player.use_item(item_id, &game.world) {
                    return;
                }

                // PREVENT borrow conflict by splitting access early
                let world_items = &game.world.items;
                let room = game.current_room();

                if let Some(def_item) = room
                    .items
                    .iter()
                    .find(|i| i.id.eq_ignore_ascii_case(item_id))
                    .and_then(|room_item| {
                        world_items
                            .iter()
                            .find(|def| def.id.eq_ignore_ascii_case(&room_item.id))
                    })
                {
                    match def_item.id.as_str() {
                        "core_terminal_input" => {
                            game.trigger_final_terminal();
                        }
                        _ => {
                            println!(
                                "{} {}",
                                "[CANNOT USE]".red(),
                                def_item.name.to_uppercase().dim()
                            );
                        }
                    }
                } else {
                    println!(
                        "{} {}",
                        "[ITEM NOT FOUND]".red().bold(),
                        item_id.to_uppercase().dim().red()
                    );
                }
            }

            Command::Help => {
                println!("{}", "[COMMAND INDEX]".green().bold());
                println!("{}", "> PRIMARY INTERFACE".green());
                println!("{}", "  look          go <dir>      take <item>".green());
                println!("{}", "  examine <item>  use <item>    inventory".green());

                println!();
                println!("{}", "> INTERACTION MODULES".green());
                println!("{}", "  talk <npc>     attack <enemy>  quests".green());
                println!("{}", "  status         help".green());

                println!();
                println!("{}", "[HIDDEN OPS ENABLED]".dim().green());
                println!("{}", "  sudo reboot    echo $USER     ls".dim().green());
                println!("{}", "  grep -r <term>".dim().green());
            }

            Command::Unknown(cmd) => match cmd.as_str() {
                "sudo reboot" => {
                    println!("{}", "[PRIVILEGED OPERATION]".yellow().bold());
                    println!("{}", "Attempting system reboot...".green());
                    std::thread::sleep(std::time::Duration::from_millis(700));
                    println!("{}", "[KERNEL PANIC]".on_green().black());
                    println!("{}", "Null pointer in sector 0xDEADBEEF".red());
                    println!("{}", "[SYSTEM HALTED]".green().bold());
                }

                "echo $USER" => {
                    println!("{}", "ghost42".green().bold());
                }

                "ls" => {
                    let room = game.current_room();
                    println!("{}", format!("📁 {}", room.name).green().bold());
                    if !room.items.is_empty() {
                        println!("{}", "[ITEMS DETECTED]".green());
                        for i in &room.items {
                            println!("{}", format!("- {}", i.id).green());
                        }
                    } else {
                        println!("{}", "- .nothing_here".dim().green());
                    }
                }

                cmd if cmd.starts_with("grep -r") => {
                    println!("{}", "[LOG TRACE ACTIVE]".green().bold());
                    println!(
                        "{}",
                        "[core/logs/0x44]: TODO: bypass dmz_auth()".green().dim()
                    );
                    println!(
                        "{}",
                        "[core/devghost/.memo]: don’t trust root.".green().dim()
                    );
                }

                _ => {
                    println!("{}", "[COMMAND NOT RECOGNIZED]".red().bold());
                    println!(
                        "{} {}",
                        ">".green(),
                        format!("'{}' is not a known interface command.", cmd)
                            .green()
                            .italic()
                    );
                    println!(
                        "{}",
                        "[TIP] Type 'help' to see available commands.".dim().green()
                    );
                }
            },
        }
    }
}

pub fn match_id_or_name<'a, T>(
    query: &str,
    objects: &'a [T],
    get_id: impl Fn(&T) -> &str,
    get_name: impl Fn(&T) -> &str,
) -> Option<&'a T> {
    let query = query.to_lowercase();
    objects.iter().find(|obj| {
        let id = get_id(obj).to_lowercase();
        let name = get_name(obj).to_lowercase();
        id == query || name == query || id.contains(&query) || name.contains(&query)
    })
}

pub fn handle_trigger_effect(effect: &str, game: &mut Game) {
    match effect {
        "spawn_syslog_wraith" => {
            if let Some(it_wing) = game.world.rooms.iter_mut().find(|r| r.id == 1) {
                if !it_wing.enemies.iter().any(|e| e.id == "syslog_wraith") {
                    it_wing.enemies.push(IdRef {
                        id: "syslog_wraith".to_string(),
                    });
                    println!("{}", "[WRAITH MATERIALIZING...]".red().bold());
                }
            }
        }

        "weaken_overheating_server" => {
            if let Some(enemy) = game
                .world
                .enemies
                .iter_mut()
                .find(|e| e.id == "overheating_server")
            {
                enemy.hp = enemy.hp.saturating_sub(8);
                println!("{}", "[THERMAL DAMPENING DEPLOYED]".green().dim());
            }
        }

        "unlock_dev_override" => {
            println!("{}", "[DEV OVERRIDE FLAG ENABLED]".green().bold());
            // You can later toggle a boolean in `game` to track this
        }

        "mark_player_corruption" => {
            println!(
                "{}",
                "[WARNING] Recursive instability detected in user profile.".red()
            );
        }

        "unlock_alternate_decision" => {
            println!("{}", "[ALTERNATE ENDING PATH UNLOCKED]".green().italic());
        }

        _ => {
            println!("{} {}", "[UNKNOWN TRIGGER EFFECT]".yellow(), effect.dim());
        }
    }
}
