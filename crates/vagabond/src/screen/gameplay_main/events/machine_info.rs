use crate::screen::gameplay_main::MachineKind;
use bevy::prelude::Event;

#[derive(Event)]
pub(crate) struct MachineInfoTrigger {
    pub(crate) kind: MachineKind,
    pub(crate) name: String,
    pub(crate) id: String,
}

impl MachineInfoTrigger {
    pub(crate) fn new(kind: MachineKind, name: String, id: String) -> Self {
        Self {
            kind,
            name,
            id,
        }
    }
}
