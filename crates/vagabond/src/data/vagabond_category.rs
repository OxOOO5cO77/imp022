use serde::{Deserialize, Serialize};

use shared_data::player::category::Category;

type NumberType = u8;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct VagabondCategory {
    pub category: Category,
    pub number: NumberType,
    pub title: String,
}
