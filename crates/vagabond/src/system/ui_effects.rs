use bevy::app::{App, Plugin, Update};

mod blinker;
mod glower;
mod hider;

pub(crate) use blinker::Blinker;
pub(crate) use glower::Glower;
pub(crate) use hider::Hider;

pub(crate) struct UiEffectsPlugin;

impl Plugin for UiEffectsPlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_systems(Update, blinker::blinker_update)
            .add_systems(Update, glower::glower_update)
            .add_systems(Update, hider::hider_update);
    }
}
