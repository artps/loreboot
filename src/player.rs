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

        println!(
            "{} -{} HP [{}]",
            "[STACK TRACE]".red().bold(),
            amount.to_string().red(),
            format!("{} HP", self.hp).green()
        );
    }

    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    pub fn add_points(&mut self, amount: u32) {
        self.points += amount;

        println!(
            "{} +{} pts [{}]",
            "[POINT INJECTION]".green().bold(),
            amount.to_string().green(),
            format!("{} total", self.points).green().dim()
        );
    }

    pub fn list_inventory(&self, world: &World) {
        if self.inventory.is_empty() {
            println!("{}", "[INVENTORY EMPTY]".green().dim());
            return;
        }

        println!("{}", "[INVENTORY DUMP]".green().bold());

        for idref in &self.inventory {
            let id = &idref.id;
            let label = format!("> mem[{}]", id).green();

            if let Some(item) = world.items.iter().find(|it| it.id == *id) {
                let type_tag = format!("[{}]", item.item_type.to_uppercase()).dim();

                let mut fx = vec![];

                if let Some(effects) = &item.effects {
                    if let Some(hp) = effects.hp {
                        fx.push(format!("+{} HP", hp).green().to_string());
                    }
                    if let Some(strength) = effects.strength {
                        fx.push(format!("+{} STR", strength).cyan().to_string());
                    }
                    if let Some(intelligence) = effects.intelligence {
                        fx.push(format!("+{} INT", intelligence).cyan().to_string());
                    }
                    if let Some(persistence) = effects.persistence {
                        fx.push(format!("+{} PERSIST", persistence).cyan().to_string());
                    }
                    if let Some(agility) = effects.agility {
                        fx.push(format!("+{} AGI", agility).cyan().to_string());
                    }
                    if let Some(charisma) = effects.charisma {
                        fx.push(format!("+{} CHA", charisma).cyan().to_string());
                    }
                    if let Some(luck) = effects.luck {
                        fx.push(format!("+{} LUCK", luck).cyan().to_string());
                    }
                    if let Some(wisdom) = effects.wisdom {
                        fx.push(format!("+{} WIS", wisdom).cyan().to_string());
                    }
                }

                let fx_summary = if fx.is_empty() {
                    "[NO EFFECT]".dim().to_string()
                } else {
                    fx.join(" ")
                };

                println!("{:<24} {} :: {}", label, type_tag, fx_summary);
            } else {
                println!("{:<24} :: {}", label, "[UNKNOWN ITEM]".yellow().dim());
            }
        }
    }

    pub fn has_item(&self, item_id: &str) -> bool {
        self.inventory.iter().any(|i| i.id == item_id)
    }

    pub fn add_item(&mut self, item_id: &str, world: &World) {
        self.inventory.push(IdRef {
            id: item_id.to_string(),
        });

        if let Some(item) = world.items.iter().find(|i| i.id == item_id) {
            if let Some(mods) = &item.effects {
                self.intelligence += mods.intelligence.unwrap_or(0);
                self.persistence += mods.persistence.unwrap_or(0);
                self.agility += mods.agility.unwrap_or(0);
                self.charisma += mods.charisma.unwrap_or(0);
                self.luck += mods.luck.unwrap_or(0);
                self.wisdom += mods.wisdom.unwrap_or(0);
                self.strength += mods.strength.unwrap_or(0);

                println!(
                    "{} {}",
                    "[TRAIT MODIFIERS APPLIED]".green().bold(),
                    item.name.to_uppercase().dim()
                );
            }
        }
    }

    pub fn remove_item(&mut self, item_id: &str, world: &World) {
        if let Some(index) = self.inventory.iter().position(|i| i.id == item_id) {
            self.inventory.remove(index);

            if let Some(item) = world.items.iter().find(|i| i.id == item_id) {
                if let Some(mods) = &item.effects {
                    self.intelligence -= mods.intelligence.unwrap_or(0);
                    self.persistence -= mods.persistence.unwrap_or(0);
                    self.agility -= mods.agility.unwrap_or(0);
                    self.charisma -= mods.charisma.unwrap_or(0);
                    self.luck -= mods.luck.unwrap_or(0);
                    self.wisdom -= mods.wisdom.unwrap_or(0);
                    self.strength -= mods.strength.unwrap_or(0);

                    println!(
                        "{} {}",
                        "[TRAIT MODIFIERS REMOVED]".yellow(),
                        item.name.to_uppercase().dim()
                    );
                }
            }
        }
    }

    pub fn use_item(&mut self, name: &str, world: &World) -> bool {
        if let Some(index) = self.inventory.iter().position(|idref| {
            world.items.iter().any(|item| {
                item.id == idref.id && item.name.to_lowercase().contains(&name.to_lowercase())
            })
        }) {
            let idref = self.inventory.remove(index);

            if let Some(item) = world.items.iter().find(|item| item.id == idref.id) {
                // Only allow certain item types to be used
                let usable_types = ["artifact", "consumable", "cursed"];
                if !usable_types.contains(&item.item_type.as_str()) {
                    println!(
                        "{}",
                        "[CANNOT USE] This item has no active function."
                            .green()
                            .dim()
                    );
                    // Return it to inventory since it wasn't actually used
                    self.inventory.insert(index, idref);
                    return false;
                }

                println!(
                    "{} {}",
                    "[USE]".green().bold(),
                    item.name.to_uppercase().green()
                );

                let mut used = false;

                if let Some(effects) = &item.effects {
                    if let Some(hp) = effects.hp {
                        self.hp = self.hp.saturating_add(hp);
                        println!(
                            "{} +{} HP",
                            "[MEMORY PATCH]".green(),
                            hp.to_string().green()
                        );
                        used = true;
                    }
                }

                if item.item_type == "cursed" {
                    println!("{}", "[CORRUPTED PAYLOAD]".red().bold());
                    println!(
                        "{}",
                        "The artifact twists your vision. You feel... less real."
                            .red()
                            .dim()
                    );
                    used = true;
                }

                if !used {
                    println!("{}", "[NO EFFECT] Nothing happened.".green().dim());
                }

                return true;
            }
        }

        println!(
            "{}",
            "[NOT FOUND] Item not in your inventory.".green().dim()
        );

        false
    }
}
