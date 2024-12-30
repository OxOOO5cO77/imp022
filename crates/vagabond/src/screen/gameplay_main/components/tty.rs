use crate::screen::gameplay_main::MachineKind;
use bevy::prelude::Component;

#[derive(Component)]
pub(crate) struct TTYMessageText {
    pub(crate) kind: MachineKind,
    pub(crate) slot: usize,
}

impl TTYMessageText {
    pub(crate) fn new(kind: MachineKind, slot: usize) -> Self {
        Self {
            kind,
            slot,
        }
    }
}
