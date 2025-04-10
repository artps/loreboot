use crate::{
    player::Player,
    structs::{Quest, QuestAction},
};
use crossterm::style::Stylize;

pub fn handle_event(
    action: QuestAction,
    target_id: &str,
    quests: &mut Vec<Quest>,
    player: &mut Player,
) {
    for quest in quests.iter_mut() {
        if quest.completed {
            continue;
        }

        if let Some(step) = quest.steps.get_mut(quest.current_step) {
            if step.action == action && step.target_id == target_id && !step.completed {
                step.completed = true;

                println!("{}", "[STEP COMPLETE]".green().bold());
                println!(
                    "{} {}",
                    ">".green(),
                    step.description.clone().green().italic()
                );

                quest.current_step += 1;

                if quest.current_step >= quest.steps.len() {
                    quest.completed = true;
                    player.quests_completed += 1;
                    player.add_points(quest.points);

                    println!();
                    println!("{}", "[QUEST COMPLETE]".green().bold());
                    println!(
                        "{} {}",
                        ">".green(),
                        quest.name.to_uppercase().green().bold()
                    );
                    println!(
                        "{} +{}",
                        "[POINT INJECTION]".green(),
                        quest.points.to_string().green()
                    );
                } else if let Some(next_step) = quest.steps.get(quest.current_step) {
                    println!();
                    println!("{}", "[NEXT OBJECTIVE]".green().bold());
                    println!("{} {}", ">".green(), next_step.description.clone().green());
                }
            }
        }
    }
}
