use serde::{Deserialize, Serialize};

use hall::core::{AttributeKind, CardNumberType, DelayType, ErgType, Host, LaunchInstruction, PriorityType, Rarity, RunInstruction, SetType};

#[derive(Clone, Serialize, Deserialize)]
pub struct VagabondCard {
    pub title: String,
    pub rarity: Rarity,
    pub number: CardNumberType,
    pub set: SetType,
    pub kind: AttributeKind,
    pub cost: ErgType,
    pub delay: DelayType,
    pub host: Host,
    pub priority: PriorityType,
    pub launch_rules: Vec<LaunchInstruction>,
    pub run_rules: Vec<RunInstruction>,
}
