use crate::screen::gameplay_main::{MachineInfo, MachineKind};
use bevy::prelude::Event;

#[derive(Event)]
pub(crate) struct MachineInfoTrigger {
    pub(crate) kind: MachineKind,
    pub(crate) info: MachineInfo,
}

impl MachineInfoTrigger {
    pub(crate) fn new(kind: MachineKind, info: MachineInfo) -> Self {
        Self {
            kind,
            info,
        }
    }
}
