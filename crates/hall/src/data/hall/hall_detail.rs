use std::mem::discriminant;

use serde::{Deserialize, Serialize};

use shared_data::game::card::CardSlot;
use shared_data::player::detail::{Detail, NumberType};

use crate::data::player::player_detail::PlayerDetail;

#[derive(Clone, Serialize, Deserialize)]
pub struct HallDetail {
    pub number: NumberType,
    pub detail: Detail,
    pub cards: Vec<CardSlot>,
}

impl HallDetail {
    pub fn is(&self, other: &Detail) -> bool {
        discriminant(&self.detail) == discriminant(other)
    }

    pub fn to_player(&self, value: &u8) -> PlayerDetail {
        PlayerDetail {
            detail: self.detail,
            number: self.number,
            value: *value,
        }
    }
}
