use bevy::prelude::Event;

use hall_lib::core::ErgArray;

#[derive(Event)]
pub(crate) struct PlayerErgTrigger {
    pub(crate) erg: ErgArray,
}

impl PlayerErgTrigger {
    pub(crate) fn new(erg: ErgArray) -> Self {
        Self {
            erg,
        }
    }
}
