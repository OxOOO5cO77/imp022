use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

pub use player_build::PlayerBuild;
pub use player_card::PlayerCard;
pub use player_detail::PlayerDetail;
pub use player_part::PlayerPart;

use shared_data::attribute::Attributes;
use shared_net::types::SeedType;

pub mod player_build;
pub mod player_card;
pub mod player_detail;
pub mod player_part;
pub mod player_state;

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub seed: SeedType,
    pub attributes: Attributes,
    pub build: [PlayerBuild; 4],
    pub detail: [PlayerDetail; 4],
    pub deck: VecDeque<PlayerCard>,
}
