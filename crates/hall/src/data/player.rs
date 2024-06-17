use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use shared_data::player::attribute::Attributes;
use shared_data::types::SeedType;
use crate::data::player_build::PlayerBuild;

use crate::data::player_card::PlayerCard;
use crate::data::player_category::PlayerCategory;

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub seed: SeedType,
    pub attributes: Attributes,
    pub build: [PlayerBuild; 4],
    pub category: [PlayerCategory; 4],
    pub deck: VecDeque<PlayerCard>,
}
