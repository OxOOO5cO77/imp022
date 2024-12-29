use crate::screen::gameplay::MachineKind;
use bevy::prelude::Event;

#[derive(Event)]
pub(crate) struct TTYMessageTrigger {
    pub(crate) kind: MachineKind,
    pub(crate) message: String,
}

impl TTYMessageTrigger {
    pub(crate) fn new(kind: MachineKind, message: &str) -> Self {
        Self {
            kind,
            message: message.to_string(),
        }
    }
}
