use crate::data::game::GameMachine;
use rand::prelude::IndexedRandom;
use rand::Rng;
use shared_data::player::attribute::{AttributeKind, AttributeValueType, Attributes};

pub struct GameRemote {
    pub attributes: Attributes,
    pub machine: GameMachine,
    pub chosen_attr: Option<AttributeKind>,
}

impl GameRemote {
    pub(crate) fn new(attributes: Attributes) -> Self {
        Self {
            attributes,
            machine: GameMachine::default(),
            chosen_attr: None,
        }
    }

    pub fn reset(&mut self) {
        self.chosen_attr = None;
    }

    pub fn choose_attr(&self, mut rng: &mut impl Rng) -> [AttributeValueType; 4] {
        const ATTR: [AttributeKind; 4] = [AttributeKind::Analyze, AttributeKind::Breach, AttributeKind::Compute, AttributeKind::Disrupt];
        let kind = self.chosen_attr.unwrap_or_else(|| *ATTR.choose(&mut rng).unwrap());
        self.attributes.get(kind)
    }
}
