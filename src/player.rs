use crate::structs::{IdRef, World};
use crossterm::style::Stylize;

#[derive(Debug, Clone)]
pub struct Player {
    pub hp: i32,
    pub intelligence: i32,
    pub persistence: i32,
    pub agility: i32,
    pub charisma: i32,
    pub luck: i32,
    pub wisdom: i32,
    pub strength: i32,
    pub inventory: Vec<IdRef>,
    pub points: u32,
    pub quests_completed: u32,
    pub status: PlayerStatus,
}

#[derive(Debug, Default, Clone)]
pub struct PlayerStatus {
    pub blocking: bool,
}

impl Player {
    pub fn new() -> Self {
        Self {
            hp: 35,
            intelligence: 5,
            persistence: 5,
            agility: 5,
            charisma: 5,
            luck: 5,
            wisdom: 5,
            strength: 5,
            inventory: Vec::new(),
            points: 0,
            quests_completed: 0,
            status: PlayerStatus::default(),
        }
    }

    pub fn take_damage(&mut self, amount: i32) {
        self.hp = (self.hp - amount).max(0);
        println!("You took {} damage. Current HP: {}", amount, self.hp);
    }

    pub fn heal(&mut self, amount: i32) {
        self.hp += amount;
        println!("You healed {} HP. Current HP: {}", amount, self.hp);
    }

    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    pub fn add_points(&mut self, amount: u32) {
        self.points += amount;
        println!("You gained {} points. Total: {}", amount, self.points);
    }

    pub fn list_inventory(&self) {
        if self.inventory.is_empty() {
            println!("{}", "[INVENTORY EMPTY]".green().dim());
            return;
        }

        println!("{}", "[INVENTORY DUMP]".green().bold());

        for item in &self.inventory {
            let id = &item.id;
            let label = format!("> mem[{}]", id).green();

            // If item details are known only by ID:
            println!("{:<24} :: {}", label, "[DATA OBJECT]".green().dim());
        }
    }

    pub fn has_item(&self, item_id: &str) -> bool {
        self.inventory.iter().any(|i| i.id == item_id)
    }

    pub fn remove_item(&mut self, item_id: &str) -> bool {
        if let Some(pos) = self.inventory.iter().position(|i| i.id == item_id) {
            self.inventory.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn use_item(&mut self, name: &str, world: &World) -> bool {
        // Find matching item in inventory by name
        if let Some(index) = self.inventory.iter().position(|idref| {
            world.items.iter().any(|item| {
                item.id == idref.id && item.name.to_lowercase().contains(&name.to_lowercase())
            })
        }) {
            let idref = self.inventory.remove(index);

            if let Some(item) = world.items.iter().find(|item| item.id == idref.id) {
                println!("{}", "[USE MODULE INITIATED]".green().bold());
                println!("{} {}", "> ITEM".green(), item.name.to_uppercase().green());

                match item.effect.as_deref() {
                    Some("restore_hp") => {
                        let amount = item.effect_amount.unwrap_or(0);
                        self.hp = self.hp.saturating_add(amount);

                        println!(
                            "{} {} → {}",
                            "[MEMORY PATCH APPLIED]".green(),
                            format!("+{} HP", amount).green().bold(),
                            self.hp.to_string().green()
                        );
                    }
                    Some(effect) => {
                        println!(
                            "{} {}",
                            "[UNKNOWN EFFECT]".yellow().bold(),
                            effect.to_uppercase().yellow()
                        );
                        println!("{}", "NO CHANGE DETECTED".dim().green());
                    }
                    None => {
                        println!(
                            "{}",
                            "[INERT MODULE] No visible effect registered.".dim().green()
                        );
                    }
                }

                return true;
            }
        }

        false
    }
}
