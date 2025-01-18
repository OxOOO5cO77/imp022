use rand::prelude::IndexedRandom;
use rand::Rng;

use crate::private::game::GameMachine;
use hall::core::{AttributeArray, AttributeKind, Attributes};

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

    pub fn end_turn(&mut self) {
        self.chosen_attr = None;
    }

    pub fn choose_attr(&self, mut rng: &mut impl Rng) -> (AttributeArray, AttributeKind) {
        const ATTR: [AttributeKind; 4] = [AttributeKind::Analyze, AttributeKind::Breach, AttributeKind::Compute, AttributeKind::Disrupt];
        let kind = self.chosen_attr.unwrap_or_else(|| *ATTR.choose(&mut rng).unwrap());
        (self.attributes.get_values(kind), kind)
    }
}
