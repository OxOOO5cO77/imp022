use serde::Deserialize;

use shared_data::player::build::{Build, NumberType};
use shared_data::player::card::CardSlot;

#[derive(Deserialize)]
pub(crate) struct BuildInstance {
    pub number: NumberType,
    pub build: Build,
    pub title: String,
    pub cards: Vec<CardSlot>,
}
