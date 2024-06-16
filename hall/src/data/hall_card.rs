use serde::{Deserialize, Serialize};

use shared_data::player::card::{CardSlot, CostType, Kind, NumberType, Rarity, SetType, Slot};

#[derive(Clone, Serialize, Deserialize)]
pub struct HallCard {
    pub rarity: Rarity,
    pub number: NumberType,
    pub set: SetType,
    pub kind: Kind,
    pub cost: CostType,
    pub rules: String,
}


impl HallCard {
    pub fn matches(&self, slot: &CardSlot) -> bool {
        let set_match = self.set == slot.0.0;
        let rarity_match = self.rarity == slot.1;
        let slot_match = match slot.2 {
            Slot::Any => true,
            Slot::Number(number) => self.number == number,
        };
        set_match && rarity_match && slot_match
    }
}
