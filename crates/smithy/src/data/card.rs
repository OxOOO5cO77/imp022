use serde::{Deserialize, Serialize};
use shared_data::game::card::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct Card {
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
