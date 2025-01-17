use bevy::prelude::Component;

use hall::core::MissionNodeKind;

#[derive(Component)]
pub(crate) struct MissionNodeLocalObserver;

#[derive(Component)]
pub(crate) struct MissionNodeButton<T> {
    pub(crate) data: T,
}

impl<T> MissionNodeButton<T> {
    pub(crate) fn new(data: T) -> Self {
        Self {
            data,
        }
    }
}

#[derive(Component)]
pub(crate) struct MissionNodeContentButton;

#[derive(Component)]
pub(crate) struct MissionNodeUserButton;

#[derive(Component)]
pub(crate) struct MissionNodeDisplay {
    pub(crate) kind: MissionNodeKind,
}

impl MissionNodeDisplay {
    pub(crate) fn new(kind: MissionNodeKind) -> Self {
        Self {
            kind,
        }
    }
}
