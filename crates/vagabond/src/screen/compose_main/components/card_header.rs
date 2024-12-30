use bevy::prelude::Component;

#[derive(Component)]
pub(crate) struct CardHeader {
    pub(crate) index: usize,
}

impl CardHeader {
    pub(crate) fn new(index: usize) -> Self {
        Self {
            index,
        }
    }
}
