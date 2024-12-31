use bevy::prelude::Component;
use hall::data::core::{MissionNodeKind, MissionNodeLinkDir};

#[derive(Component)]
pub(crate) struct MissionNodeLinkButton {
    pub(crate) dir: MissionNodeLinkDir,
}

impl MissionNodeLinkButton {
    pub(crate) fn new(dir: MissionNodeLinkDir) -> Self {
        Self {
            dir,
        }
    }
}

#[derive(Component)]
pub(crate) struct MissionNodeContentButton;

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
