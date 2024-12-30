use bevy::prelude::Component;

#[derive(Component)]
pub(crate) struct RollText {
    pub(crate) index: usize,
}

impl RollText {
    pub(crate) fn new(index: usize) -> Self {
        Self {
            index,
        }
    }
}
