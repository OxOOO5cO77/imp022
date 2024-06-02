use std::collections::VecDeque;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::player::attribute::Attributes;
use crate::player::build::Build;
use crate::player::card::Card;
use crate::player::category::Category;

pub mod attribute;
pub mod build;
pub mod card;
pub mod part;
pub mod category;

#[derive(Serialize, Deserialize)]
pub struct PlayerBuild {
    pub build: Build,
    pub title: String,
    pub value: u8,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerCategory {
    pub category: Category,
    pub title: String,
    pub value: u8,
}

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub attributes: Attributes,
    pub build: [PlayerBuild; 4],
    pub category: [PlayerCategory; 4],
    pub deck: VecDeque<Card>,
    pub seed: u64,
    pub name: String,
    pub id: String,
    pub birthplace: (String, String, String),
    pub dob: NaiveDate,
}

impl Player {
    pub fn birthplace(&self) -> String {
        format!("{},{},{}", self.birthplace.0, self.birthplace.1, self.birthplace.2)
    }

    pub fn age(&self) -> u32 {
        NaiveDate::from_yo_opt(2049, 1).unwrap().years_since(self.dob).unwrap()
    }
}
