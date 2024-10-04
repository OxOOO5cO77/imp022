use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use shared_data::player::attribute::Attributes;
use shared_data::types::SeedType;
use player_build::PlayerBuild;

use player_card::PlayerCard;
use player_category::PlayerCategory;

pub mod player_part;
pub mod player_card;
pub mod player_build;
pub mod player_category;

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub seed: SeedType,
    pub attributes: Attributes,
    pub build: [PlayerBuild; 4],
    pub category: [PlayerCategory; 4],
    pub deck: VecDeque<PlayerCard>,
}
