use std::mem::discriminant;

use imp022_shared::player::card::CardSlot;
use imp022_shared::player::category::Category;
use imp022_shared::player::PlayerCategory;
use serde::Deserialize;
use crate::data::player::player_builder::PlayerPartBuilder;

#[derive(Clone, Deserialize)]
pub(crate) struct CategoryInstance {
    pub(crate) category: Category,
    pub(crate) title: String,
    pub(crate) cards: Vec<CardSlot>,
}

impl CategoryInstance {
    pub(crate) fn is(&self, other: &Category) -> bool {
        discriminant(&self.category) == discriminant(other)
    }

    fn to_player(&self, value: &u8) -> PlayerCategory {
        PlayerCategory {
            category: self.category,
            title: self.title.clone(),
            value: *value,
        }
    }

    pub(crate) fn from_parts(category: &Option<PlayerPartBuilder>, values: &Option<PlayerPartBuilder>) -> Option<[PlayerCategory; 4]> {
        let category = category.as_ref()?;
        let values = values.as_ref()?;
        Some([
            category.category[0].to_player(&values.values[0]),
            category.category[1].to_player(&values.values[1]),
            category.category[2].to_player(&values.values[2]),
            category.category[3].to_player(&values.values[3]),
        ])
    }

}
