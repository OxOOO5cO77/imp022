use std::mem::discriminant;

use serde::{Deserialize, Serialize};

use shared_data::player::build::{Build, NumberType};
use shared_data::game::card::CardSlot;
use crate::data::player::player_build::PlayerBuild;

#[derive(Clone, Serialize, Deserialize)]
pub struct HallBuild {
    pub number: NumberType,
    pub build: Build,
    pub cards: Vec<CardSlot>,
}

impl HallBuild {
    pub fn is(&self, other: &Build) -> bool {
        discriminant(&self.build) == discriminant(other)
    }

    pub fn to_player(&self, value: &u8) -> PlayerBuild {
        PlayerBuild {
            build: self.build,
            number: self.number,
            value: *value,
        }
    }
}
