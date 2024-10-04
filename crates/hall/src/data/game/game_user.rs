use rand::{thread_rng, Rng};
use crate::data::player::Player;
use shared_data::types::{AuthType, PartType};
use crate::data::game::GameMachine;

pub struct GameUser {
    pub(crate) auth: AuthType,
    pub parts: Vec<PartType>,
    player: Option<Player>,
    pub(crate) machine: GameMachine,
}

impl GameUser {
    pub fn new(auth: AuthType) -> Self{
        Self  {
            auth,
            parts: thread_rng().gen_iter().take(8).collect(),
            player: None,
            machine: GameMachine::default()
        }
    }
}
