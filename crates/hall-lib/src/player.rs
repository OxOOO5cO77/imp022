use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

use shared_net::SeedType;

use crate::core::Attributes;

mod player_build;
mod player_card;
mod player_detail;
mod player_part;

pub use player_build::PlayerBuild;
pub use player_card::PlayerCard;
pub use player_detail::PlayerDetail;
pub use player_part::PlayerPart;

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub seed: SeedType,
    pub attributes: Attributes,
    pub build: [PlayerBuild; 4],
    pub detail: [PlayerDetail; 4],
    pub deck: VecDeque<PlayerCard>,
}
