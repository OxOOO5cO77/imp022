use bevy::prelude::{Component, Vec2};

#[derive(Component)]
pub(crate) struct UiFxTrackedSize {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

impl From<Vec2> for UiFxTrackedSize {
    fn from(size: Vec2) -> Self {
        Self {
            x: size.x,
            y: size.y,
        }
    }
}
