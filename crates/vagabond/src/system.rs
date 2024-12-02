use crate::system::glower::GlowerPlugin;
use bevy::app::{PluginGroup, PluginGroupBuilder};

mod app_state;
mod glower;

pub(crate) use app_state::AppState;
pub(crate) use glower::Glower;

pub(super) struct SystemPlugins;

impl PluginGroup for SystemPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>() //
            .add(GlowerPlugin)
    }
}
