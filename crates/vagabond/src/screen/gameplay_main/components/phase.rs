use crate::screen::gameplay_main::VagabondGamePhase;
use bevy::prelude::Component;

#[derive(Component)]
pub(crate) struct PhaseIcon {
    pub(crate) phase: VagabondGamePhase,
}

impl PhaseIcon {
    pub(crate) fn new(phase: VagabondGamePhase) -> Self {
        Self {
            phase,
        }
    }
}
