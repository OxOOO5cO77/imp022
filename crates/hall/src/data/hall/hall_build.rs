use std::mem::discriminant;

use serde::{Deserialize, Serialize};

use crate::data::player::player_build::PlayerBuild;
use shared_data::game::card::CardSlot;
use shared_data::player::build::{Build, CompanyType, MarketType, NumberType};

#[derive(Clone, Serialize, Deserialize)]
pub struct HallBuild {
    pub number: NumberType,
    pub build: Build,
    pub cards: Vec<CardSlot>,
}

impl HallBuild {
    pub fn is(&self, other: impl FnOnce(CompanyType, MarketType) -> Build) -> bool {
        discriminant(&self.build) == discriminant(&other(0, 0))
    }

    pub fn to_player(&self, value: &u8) -> PlayerBuild {
        PlayerBuild {
            build: self.build,
            number: self.number,
            value: *value,
        }
    }
}
