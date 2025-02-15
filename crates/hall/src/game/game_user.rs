use rand::Rng;

use hall_lib::player::Player;
use shared_net::{AuthType, PartType};

use crate::game::{GameMachine, GameUserMissionState, GameUserState};

pub struct GameUser {
    pub(crate) auth: AuthType,
    pub parts: Vec<PartType>,
    pub player: Option<Player>,
    pub machine: GameMachine,
    pub state: GameUserState,
    pub mission_state: GameUserMissionState,
}

impl GameUser {
    pub fn new(auth: AuthType) -> Self {
        Self {
            auth,
            parts: rand::rng().random_iter().take(8).collect(),
            player: None,
            machine: GameMachine::default(),
            state: GameUserState::default(),
            mission_state: GameUserMissionState::default(),
        }
    }
}
