use serde::{Deserialize, Serialize};
use shared_data::attribute::AttributeKind;
use shared_data::card::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct VagabondCard {
    pub title: String,
    pub rarity: Rarity,
    pub number: CardNumberType,
    pub set: SetType,
    pub kind: AttributeKind,
    pub cost: ErgType,
    pub delay: DelayType,
    pub priority: PriorityType,
    pub launch_rules: String,
    pub run_rules: String,
}
