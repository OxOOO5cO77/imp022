use serde::{Deserialize, Serialize};

pub type SetType = u8;
pub type NumberType = u8;
pub type CostType = u16;

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Set(pub SetType);

#[derive(Default, Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rarity {
    #[default] Common,
    Uncommon,
    Rare,
    Legendary,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub enum Slot {
    Any,
    Number(NumberType),
}

#[derive(Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum Kind {
    Analyze,
    Breach,
    Compute,
    Disrupt,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CardSlot(pub Set, pub Rarity, pub Slot);
