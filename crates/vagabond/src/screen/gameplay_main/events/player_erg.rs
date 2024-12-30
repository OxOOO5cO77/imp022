use bevy::prelude::Event;
use hall::data::core::ErgType;

#[derive(Event)]
pub(crate) struct PlayerErgTrigger {
    pub(crate) erg: [ErgType; 4],
}

impl PlayerErgTrigger {
    pub(crate) fn new(erg: [ErgType; 4]) -> Self {
        Self {
            erg,
        }
    }
}
