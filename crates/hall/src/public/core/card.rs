use crate::core::types::{CardNumberType, SetType};
use num_enum::{FromPrimitive, IntoPrimitive};
use serde::{Deserialize, Serialize};
use shared_net::{Bufferable, SizedBuffer, SizedBufferError};

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

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Host {
    None,
    Local,
    Remote,
}

pub type ActorIndexType = u8;

#[derive(Clone, Copy)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum PickedCardTarget {
    None,
    MachineLocal,
    MachineRemote,
    Actor(ActorIndexType),
}

impl Bufferable for PickedCardTarget {
    fn push_into(&self, buf: &mut SizedBuffer) -> Result<usize, SizedBufferError> {
        match self {
            PickedCardTarget::None => 0u8,
            PickedCardTarget::MachineLocal => 1,
            PickedCardTarget::MachineRemote => 2,
            PickedCardTarget::Actor(index) => 3 + index,
        }
        .push_into(buf)
    }

    fn pull_from(buf: &mut SizedBuffer) -> Result<Self, SizedBufferError> {
        let result = match u8::pull_from(buf)? {
            0 => PickedCardTarget::None,
            1 => PickedCardTarget::MachineLocal,
            2 => PickedCardTarget::MachineRemote,
            index @ 3..12 => PickedCardTarget::Actor(index - 3),
            _ => PickedCardTarget::None,
        };
        Ok(result)
    }

    fn size_in_buffer(&self) -> usize {
        size_of::<u8>()
    }
}
