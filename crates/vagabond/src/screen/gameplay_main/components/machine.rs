use bevy::prelude::Component;

use hall::core::DelayType;

#[derive(Component)]
pub(crate) struct MachineQueueItem {
    pub(crate) delay: DelayType,
}

impl MachineQueueItem {
    pub(crate) fn new(delay: DelayType) -> Self {
        Self {
            delay,
        }
    }
}

#[derive(Component)]
pub(crate) enum MachineTextKind {
    Title,
    Id,
    Vitals(usize),
}

#[derive(Component)]
pub(crate) struct MachineText {
    pub(crate) kind: MachineTextKind,
}

impl MachineText {
    pub(crate) fn new(kind: MachineTextKind) -> Self {
        Self {
            kind,
        }
    }
}

#[derive(Component)]
pub(crate) struct MachineRunning {
    pub(crate) index: usize,
}

impl MachineRunning {
    pub(crate) fn new(index: usize) -> Self {
        Self {
            index,
        }
    }
}

#[derive(Component, Copy, Clone, Hash, Eq, PartialEq)]
pub(crate) enum MachineKind {
    Local,
    Remote,
}
