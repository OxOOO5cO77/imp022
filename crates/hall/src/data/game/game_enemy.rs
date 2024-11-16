use crate::data::game::GameMachine;
use shared_data::player::attribute::Attributes;

pub struct GameEnemy {
    pub attributes: Attributes,
    pub machine: GameMachine,
}

impl GameEnemy {
    pub fn new(attributes: Attributes) -> Self {
        Self {
            attributes,
            machine: GameMachine::default(),
        }
    }
}
