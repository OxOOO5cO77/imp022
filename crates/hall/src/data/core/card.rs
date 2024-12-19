use num_enum::{FromPrimitive, IntoPrimitive};
use serde::{Deserialize, Serialize};

pub type SetType = u8;
pub type CardNumberType = u8;
pub type ErgType = u8;
pub type PriorityType = u8;
pub type DelayType = u8;

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Set(pub SetType);

#[repr(u8)]
#[derive(Default, Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, FromPrimitive, IntoPrimitive)]
pub enum Rarity {
    #[default]
    Common,
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
