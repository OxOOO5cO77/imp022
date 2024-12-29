use crate::screen::gameplay::VagabondGamePhase;
use bevy::prelude::Event;

#[derive(Event)]
pub(crate) struct GamePhaseTrigger {
    pub(crate) phase: VagabondGamePhase,
}

impl GamePhaseTrigger {
    pub(crate) fn new(phase: VagabondGamePhase) -> Self {
        Self {
            phase,
        }
    }
}
