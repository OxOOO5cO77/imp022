use serde::{Deserialize, Serialize};

pub type SetType = u8;
pub type CardNumberType = u8;
pub type ErgType = u16;
pub type PriorityType = u8;
pub type DelayType = u8;

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
    Number(CardNumberType),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CardSlot(pub Set, pub Rarity, pub Slot);
