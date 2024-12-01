use std::mem::discriminant;

use serde::{Deserialize, Serialize};

use crate::data::player::PlayerBuild;
use shared_data::build::{Build, BuildNumberType, CompanyType, MarketType};
use shared_data::card::CardSlot;

#[derive(Clone, Serialize, Deserialize)]
pub struct HallBuild {
    pub number: BuildNumberType,
    pub build: Build,
    pub cards: Vec<CardSlot>,
}

impl HallBuild {
    pub fn is(&self, other: impl FnOnce(CompanyType, MarketType) -> Build) -> bool {
        discriminant(&self.build) == discriminant(&other(0, 0))
    }

    pub fn to_player(&self, value: u8) -> PlayerBuild {
        PlayerBuild {
            build: self.build,
            number: self.number,
            value,
        }
    }
}
