use serde::{Deserialize, Serialize};

use crate::player::Player;

#[derive(Serialize, Deserialize)]
pub struct PlayerRequest {
    pub access: u64,
    pub breach: u64,
    pub compute: u64,
    pub disrupt: u64,
    pub build: u64,
    pub build_values: u64,
    pub category: u64,
    pub category_values: u64,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerResponse {
    pub player: Option<Player>,
}
