use serde::{Deserialize, Serialize};

use crate::player::part::PlayerPart;

#[derive(Serialize, Deserialize)]
pub struct PartRequest {
    pub seeds: [u64; 8],
}

#[derive(Default, Serialize, Deserialize)]
pub struct PartResponse {
    pub parts: Vec<PlayerPart>,
}

