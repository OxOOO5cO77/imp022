use crate::data::core::Attributes;
use serde::{Deserialize, Serialize};
use shared_net::types::SeedType;
use std::collections::VecDeque;

mod player_build;
mod player_card;
mod player_detail;
mod player_mission_state;
mod player_part;
mod player_state;

pub use player_build::PlayerBuild;
pub use player_card::PlayerCard;
pub use player_detail::PlayerDetail;
pub use player_mission_state::PlayerMissionState;
pub use player_part::PlayerPart;
pub use player_state::{PlayerCommandState, PlayerState, PlayerStatePlayerView};

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub seed: SeedType,
    pub attributes: Attributes,
    pub build: [PlayerBuild; 4],
    pub detail: [PlayerDetail; 4],
    pub deck: VecDeque<PlayerCard>,
}
