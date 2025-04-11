use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct World {
    pub rooms: Vec<Room>,
    pub items: Vec<Item>,
    pub enemies: Vec<Enemy>,
    pub npcs: Vec<Npc>,
    pub quests: Vec<Quest>,
}

// ========== Rooms and Navigation ==========

#[derive(Debug, Deserialize, Clone)]
pub struct Room {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub points: u32,
    pub exits: Vec<Exit>,
    pub items: Vec<IdRef>,
    pub enemies: Vec<IdRef>,
    pub npcs: Vec<IdRef>,
    pub lore: Vec<LoreObject>,
    pub role: Option<String>, // e.g., "boot", "storage", "network", "core", "interface"
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoreObject {
    pub id: String,
    pub name: String,
    pub description: String,
    pub logs: Vec<String>,
    pub hidden: bool, // requires `scan` to be visible
    pub generated: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Exit {
    pub direction: String,
    pub to: usize,
    pub locked: Option<bool>,
    pub required_item_id: Option<String>,
    pub required_quest_id: Option<String>,
    pub required_step: Option<String>,
    pub locked_msg: Option<String>,
    pub unlock_hint: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct IdRef {
    pub id: String,
}

// ========== Items ==========

#[derive(Debug, Deserialize, Clone)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub points: u32,
    pub effects: Option<Effects>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Effects {
    pub hp: Option<i32>,
    pub strength: Option<i32>,
    pub intelligence: Option<i32>,
    pub persistence: Option<i32>,
    pub agility: Option<i32>,
    pub charisma: Option<i32>,
    pub luck: Option<i32>,
    pub wisdom: Option<i32>,
}

// ========== Enemies ==========

#[derive(Debug, Deserialize, Clone)]
pub struct Enemy {
    pub id: String,
    pub name: String,
    pub description: String,
    pub hp: i32,
    pub strength: i32,
    pub points: u32,
}

// ========== NPCs ==========

#[derive(Debug, Deserialize, Clone)]
pub struct Npc {
    pub id: String,
    pub name: String,
    pub dialogue: String,
    pub quest_to_give: Option<String>,
    pub points: u32,
}

// ========== Quests ==========

#[derive(Debug, Deserialize, Clone)]
pub struct Quest {
    pub id: String,
    pub name: String,
    pub description: String,
    pub points: u32,
    pub current_step: usize,
    pub completed: bool,
    pub steps: Vec<QuestStep>,
    pub required_for_ending: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct QuestStep {
    pub id: String,
    pub description: String,
    pub action: QuestAction,
    pub target_id: String,
    pub completed: bool,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum QuestAction {
    DefeatEnemy,
    CollectItem,
    TalkNpc,
}
