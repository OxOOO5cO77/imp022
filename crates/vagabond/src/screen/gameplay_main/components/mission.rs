use bevy::prelude::Component;
use hall::data::core::MissionNodeKind;

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
