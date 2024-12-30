use bevy::prelude::Component;

#[derive(Component)]
pub(crate) struct RemoteAttrText {
    pub(crate) index: usize,
}

impl RemoteAttrText {
    pub(crate) fn new(index: usize) -> Self {
        Self {
            index,
        }
    }
}

#[derive(Component)]
pub(crate) struct RemoteAttrIcon;
