use std::collections::VecDeque;

use chrono::NaiveDate;
use rand::prelude::*;

use crate::data::data_manager::DataManager;
use crate::data::player::attribute::Attributes;
use crate::data::player::build::BuildInstance;
use crate::data::player::card::Card;
use crate::data::player::category::CategoryInstance;

mod attribute;
pub mod build;
pub(crate) mod card;
pub mod category;
pub(crate) mod player_builder;

pub(crate) struct Player {
    pub(crate) attributes: Attributes,
    pub(crate) build: [(BuildInstance, u8); 4],
    pub(crate) category: [(CategoryInstance, u8); 4],
    pub(crate) deq: VecDeque<Card>,
    pub(crate) seed: u64,
    pub(crate) name: String,
    pub(crate) id: String,
    pub(crate) birthplace: (String, String, String),
    pub(crate) dob: NaiveDate,
}

impl Player {
    pub(crate) fn fill_deq(&mut self, rng: &mut impl Rng, dm: &DataManager) {
        self.deq = VecDeque::new();

        let build_cards = self.build.iter().flat_map(|o| dm.pick_cards(rng, &o.0.cards, o.1));
        self.deq.extend(build_cards);

        let category_cards = self.category.iter().flat_map(|o| dm.pick_cards(rng, &o.0.cards, o.1));
        self.deq.extend(category_cards);
        self.sort_deq_rarity();
    }
    // pub(crate) fn shuffle_deq(&mut self, rng: &mut impl Rng) {
    //     self.deq.make_contiguous().shuffle(rng)
    // }

    fn sort_deq_rarity(&mut self) {
        self.deq.make_contiguous().sort_by(|a, b| b.id.1.cmp(&a.id.1));
    }

    pub(crate) fn birthplace(&self) -> String {
        format!("{},{},{}", self.birthplace.0, self.birthplace.1, self.birthplace.2)
    }

    pub(crate) fn age(&self) -> u32 {
        NaiveDate::from_yo_opt(2049, 1).unwrap().years_since(self.dob).unwrap()
    }
}
