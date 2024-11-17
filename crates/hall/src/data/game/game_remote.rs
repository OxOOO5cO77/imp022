use crate::data::game::GameMachine;
use shared_data::player::attribute::Attributes;

pub struct GameRemote {
    pub attributes: Attributes,
    pub machine: GameMachine,
}

impl GameRemote {
    pub fn new(attributes: Attributes) -> Self {
        Self {
            attributes,
            machine: GameMachine::default(),
        }
    }
}
