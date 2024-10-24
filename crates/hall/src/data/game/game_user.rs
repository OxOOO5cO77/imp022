use crate::data::game::GameMachine;
use crate::data::player::player_state::PlayerState;
use crate::data::player::Player;
use rand::{thread_rng, Rng};
use shared_data::types::{AuthType, PartType};

pub struct GameUser {
    pub(crate) auth: AuthType,
    pub parts: Vec<PartType>,
    pub player: Option<Player>,
    pub machine: GameMachine,
    pub state: PlayerState,
}

impl GameUser {
    pub fn new(auth: AuthType) -> Self {
        Self {
            auth,
            parts: thread_rng().gen_iter().take(8).collect(),
            player: None,
            machine: GameMachine::default(),
            state: PlayerState::default(),
        }
    }
}
