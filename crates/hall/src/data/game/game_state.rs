use crate::data::game::{GameMachine, GameUser};
use shared_data::types::{AuthType, UserType};
use std::collections::hash_map::Entry;
use std::collections::HashMap;

type CpuType = u128;


#[derive(Default)]
pub struct GameState {
    users: HashMap<UserType, GameUser>,
    cpus: HashMap<CpuType, GameMachine>,
    current_tick: u32,
}

impl GameState {
    pub fn user_add(&mut self, user_type: UserType, game_user: GameUser) {
        self.users.insert(user_type, game_user);
    }

    pub fn user_remove(&mut self, user_type: UserType, user_auth: AuthType) {
        match self.users.entry(user_type) {
            Entry::Occupied(user) if user.get().auth == user_auth => self.users.remove(&user_type),
            _ => None,
        };
    }

    pub fn is_empty(&self) -> bool {
        self.users.is_empty()
    }

    pub fn tick(&mut self) {
        self.current_tick += 1;

        self.users.values_mut().for_each(|user| user.machine.tick());
        self.cpus.values_mut().for_each(|machine| machine.tick());
    }
}
