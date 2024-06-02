use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Set(u8);

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Legendary,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
enum Slot {
    Any,
    Number(u8),
}

#[derive(Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum Kind {
    Analyze,
    Breach,
    Compute,
    Disrupt,
}

#[derive(Clone, Deserialize)]
pub struct CardSlot(Set, Rarity, Slot);

#[derive(Clone, Serialize, Deserialize)]
pub struct Card {
    pub id: (Set, Rarity, u8),
    pub name: String,
    pub text: String,
    pub kind: Kind,
    pub cost: u32,
}

impl Card {
    pub fn matches(&self, slot: &CardSlot) -> bool {
        let set_match = self.id.0 == slot.0;
        let rarity_match = self.id.1 == slot.1;
        let slot_match = match slot.2 {
            Slot::Any => true,
            Slot::Number(number) => self.id.2 == number,
        };
        set_match && rarity_match && slot_match
    }
}
