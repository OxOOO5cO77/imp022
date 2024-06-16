use serde::{Deserialize, Serialize};

use crate::data::player_bio::PlayerBio;

#[derive(Serialize, Deserialize)]
pub struct PlayerBioResponse {
    pub player_bio: Option<PlayerBio>,
}
