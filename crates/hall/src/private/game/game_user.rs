use rand::Rng;

use hall::player::{Player, PlayerMissionState, PlayerState};
use shared_net::{AuthType, PartType};

use crate::private::game::GameMachine;

pub struct GameUser {
    pub(crate) auth: AuthType,
    pub parts: Vec<PartType>,
    pub player: Option<Player>,
    pub machine: GameMachine,
    pub state: PlayerState,
    pub mission_state: PlayerMissionState,
}

impl GameUser {
    pub fn new(auth: AuthType) -> Self {
        Self {
            auth,
            parts: rand::rng().random_iter().take(8).collect(),
            player: None,
            machine: GameMachine::default(),
            state: PlayerState::default(),
            mission_state: PlayerMissionState::default(),
        }
    }
}
