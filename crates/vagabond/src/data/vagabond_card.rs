use hall::data::core::{AttributeKind, CardNumberType, DelayType, ErgType, Instruction, PriorityType, Rarity, SetType};
use serde::{Deserialize, Serialize};

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
    pub launch_rules: Vec<Instruction>,
    pub run_rules: Vec<Instruction>,
}
