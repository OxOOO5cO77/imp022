use serde::Deserialize;

use shared_data::game::card::CardSlot;
use shared_data::player::category::{Category, NumberType};

#[derive(Deserialize)]
pub struct CategoryInstance {
    pub number: NumberType,
    pub category: Category,
    pub title: String,
    pub cards: Vec<CardSlot>,
}
