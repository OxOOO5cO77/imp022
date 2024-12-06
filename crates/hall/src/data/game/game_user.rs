use crate::data::game::game_state::RemoteIdType;
use crate::data::game::GameMachine;
use crate::data::player::{Player, PlayerState};
use rand::Rng;
use shared_net::types::{AuthType, PartType};

pub struct GameUser {
    pub(crate) auth: AuthType,
    pub parts: Vec<PartType>,
    pub player: Option<Player>,
    pub machine: GameMachine,
    pub state: PlayerState,
    pub remote: Option<RemoteIdType>,
}

impl GameUser {
    pub fn new(auth: AuthType) -> Self {
        Self {
            auth,
            parts: rand::rng().random_iter().take(8).collect(),
            player: None,
            machine: GameMachine::default(),
            state: PlayerState::default(),
            remote: None,
        }
    }
}
