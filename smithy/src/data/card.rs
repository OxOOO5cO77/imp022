use serde::{Deserialize, Serialize};
use shared_data::player::card::{CostType, Kind, NumberType, Rarity, SetType};

#[derive(Clone, Serialize, Deserialize)]
pub struct Card {
    pub title: String,
    pub rarity: Rarity,
    pub number: NumberType,
    pub set: SetType,
    pub kind: Kind,
    pub cost: CostType,
    pub rules: String,
}
