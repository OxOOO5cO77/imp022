use std::mem::discriminant;

use serde::{Deserialize, Serialize};

use crate::data::player::PlayerDetail;
use shared_data::card::CardSlot;
use shared_data::detail::{Detail, DetailNumberType, GeneralType, SpecificType};

#[derive(Clone, Serialize, Deserialize)]
pub struct HallDetail {
    pub number: DetailNumberType,
    pub detail: Detail,
    pub cards: Vec<CardSlot>,
}

impl HallDetail {
    pub fn is(&self, other: impl FnOnce(GeneralType, SpecificType) -> Detail) -> bool {
        discriminant(&self.detail) == discriminant(&other(0, 0))
    }

    pub fn to_player(&self, value: &u8) -> PlayerDetail {
        PlayerDetail {
            detail: self.detail,
            number: self.number,
            value: *value,
        }
    }
}
