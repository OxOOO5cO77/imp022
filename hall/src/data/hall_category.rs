use std::mem::discriminant;

use serde::{Deserialize, Serialize};

use shared_data::player::card::CardSlot;
use shared_data::player::category::{Category, NumberType};

use crate::data::player_category::PlayerCategory;

#[derive(Clone, Serialize, Deserialize)]
pub struct HallCategory {
    pub number: NumberType,
    pub category: Category,
    pub cards: Vec<CardSlot>,
}

impl HallCategory {
    pub fn is(&self, other: &Category) -> bool {
        discriminant(&self.category) == discriminant(other)
    }

    pub fn to_player(&self, value: &u8) -> PlayerCategory {
        PlayerCategory {
            category: self.category,
            number: self.number,
            value: *value,
        }
    }
}
