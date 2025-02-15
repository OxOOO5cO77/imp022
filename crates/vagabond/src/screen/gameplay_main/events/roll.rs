use bevy::prelude::Event;
use hall_lib::core::ErgArray;

#[derive(Event)]
pub(crate) struct RollTrigger {
    pub(crate) roll: ErgArray,
}

impl RollTrigger {
    pub(crate) fn new(roll: ErgArray) -> Self {
        Self {
            roll,
        }
    }
}
