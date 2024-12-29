use bevy::prelude::Event;
use hall::data::core::ErgType;

#[derive(Event)]
pub(crate) struct RollTrigger {
    pub(crate) roll: [ErgType; 4],
}

impl RollTrigger {
    pub(crate) fn new(roll: [ErgType; 4]) -> Self {
        Self {
            roll,
        }
    }
}
