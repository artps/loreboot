use crate::{
    player::Player,
    structs::{Quest, World},
};

pub struct Game {
    pub world: World,
    pub player: Player,
    pub current_room: usize,
    pub quests: Vec<Quest>,
}

impl Game {
    pub fn new(world: World) -> Self {
        Self {
            player: Player::new(),
            current_room: 0, // Start in the Lobby
            quests: Vec::new(),
            world,
        }
    }

    pub fn current_room(&self) -> &crate::structs::Room {
        &self.world.rooms[self.current_room]
    }

    pub fn current_room_mut(&mut self) -> &mut crate::structs::Room {
        &mut self.world.rooms[self.current_room]
    }
}

unsafe impl Send for Game {}
