use std::collections::VecDeque;

use chrono::NaiveDate;
use serde::Deserialize;

use crate::data::player::attribute::Attributes;
use crate::data::player::build::Build;
use crate::data::player::card::Card;
use crate::data::player::category::Category;

pub(crate) mod attribute;
pub(crate) mod build;
pub(crate) mod card;
pub(crate) mod category;
pub(crate) mod part;

#[derive(Deserialize)]
pub(crate) struct PlayerBuild {
    build: Build,
    title: String,
    value: u8,
}

#[derive(Deserialize)]
pub(crate) struct PlayerCategory {
    category: Category,
    title: String,
    value: u8,
}

#[derive(Deserialize)]
pub(crate) struct Player {
    pub(crate) attributes: Attributes,
    pub(crate) build: [PlayerBuild; 4],
    pub(crate) category: [PlayerCategory; 4],
    pub(crate) deck: VecDeque<Card>,
    pub(crate) seed: u64,
    pub(crate) name: String,
    pub(crate) id: String,
    pub(crate) birthplace: (String, String, String),
    pub(crate) dob: NaiveDate,
}


impl Player {
    pub(crate) fn birthplace(&self) -> String {
        format!("{},{},{}", self.birthplace.0, self.birthplace.1, self.birthplace.2)
    }

    pub(crate) fn age(&self) -> u32 {
        NaiveDate::from_yo_opt(2049, 1).unwrap().years_since(self.dob).unwrap()
    }
}
