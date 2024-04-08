use serde::Deserialize;

#[derive(Clone, Deserialize, PartialEq)]
pub(crate) struct Set(u8);

#[derive(Clone, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Rarity {
    Common,
    Uncommon,
    Rare,
    Legendary,
}

#[derive(Clone, Deserialize, PartialEq)]
enum Slot {
    Any,
    Number(u8),
}

#[derive(Clone, Copy, Deserialize, Hash, PartialEq, Eq)]
pub(crate) enum Kind {
    Analyze,
    Breach,
    Compute,
    Disrupt,
}

#[derive(Clone, Deserialize)]
pub(crate) struct CardSlot(Set, Rarity, Slot);

#[derive(Clone, Deserialize)]
pub(crate) struct Card {
    pub(crate) id: (Set, Rarity, u8),
    pub(crate) name: String,
    pub(crate) text: String,
    pub(crate) kind: Kind,
    pub(crate) cost: u32,
}

impl Card {
    pub(crate) fn matches(&self, slot: &CardSlot) -> bool {
        let set_match = self.id.0 == slot.0;
        let rarity_match = self.id.1 == slot.1;
        let slot_match = match slot.2 {
            Slot::Any => true,
            Slot::Number(number) => self.id.2 == number,
        };
        set_match && rarity_match && slot_match
    }
}
