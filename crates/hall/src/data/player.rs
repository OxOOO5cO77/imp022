use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use player_build::PlayerBuild;
use shared_data::player::attribute::Attributes;
use shared_data::types::SeedType;

use player_card::PlayerCard;
use player_category::PlayerCategory;
use shared_data::game::card::Kind;

pub mod player_part;
pub mod player_card;
pub mod player_build;
pub mod player_category;
pub mod player_state;

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub seed: SeedType,
    pub attributes: Attributes,
    pub build: [PlayerBuild; 4],
    pub category: [PlayerCategory; 4],
    pub deck: VecDeque<PlayerCard>,
}

impl Player {
    pub fn get_attr(&self, kind: Kind) -> [u8; 4] {
        match kind {
            Kind::Analyze => self.attributes.analyze.to_array(),
            Kind::Breach => self.attributes.breach.to_array(),
            Kind::Compute => self.attributes.compute.to_array(),
            Kind::Disrupt => self.attributes.disrupt.to_array(),
        }
    }
}