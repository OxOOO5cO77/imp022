use serde::{Deserialize, Serialize};

use crate::data::player::PlayerCard;
use shared_data::attribute::AttributeKind;
use shared_data::card::*;
use shared_data::instruction::Instruction;

#[derive(Clone, Serialize, Deserialize)]
pub struct HallCard {
    pub rarity: Rarity,
    pub number: CardNumberType,
    pub set: SetType,
    pub kind: AttributeKind,
    pub cost: ErgType,
    pub delay: DelayType,
    pub launch_code: Vec<Instruction>,
    pub priority: PriorityType,
    pub run_code: Vec<Instruction>,
}

impl HallCard {
    pub fn matches(&self, slot: &CardSlot) -> bool {
        let set_match = self.set == slot.0 .0;
        let rarity_match = self.rarity == slot.1;
        let slot_match = match slot.2 {
            Slot::Any => true,
            Slot::Number(number) => self.number == number,
        };
        set_match && rarity_match && slot_match
    }

    pub fn to_player_card(&self) -> PlayerCard {
        PlayerCard {
            rarity: self.rarity,
            number: self.number,
            set: self.set,
        }
    }
}
