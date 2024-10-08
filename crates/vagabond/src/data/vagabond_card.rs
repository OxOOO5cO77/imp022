use serde::{Deserialize, Serialize};
use shared_data::game::card::*;

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
    pub queue: DelayType,
    pub priority: PriorityType,
    pub launch_rules: String,
    pub run_rules: String,
}
