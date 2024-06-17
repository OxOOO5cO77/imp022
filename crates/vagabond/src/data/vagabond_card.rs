use serde::{Deserialize, Serialize};

use shared_data::player::card::{Kind, Rarity};

type SetType = u8;
type NumberType = u8;
type CostType = u16;

#[derive(Clone, Serialize, Deserialize)]
pub struct VagabondCard {
    pub title: String,
    pub rarity: Rarity,
    pub number: NumberType,
    pub set: SetType,
    pub kind: Kind,
    pub cost: CostType,
    pub rules: String,
}
