use crate::system::glower::GlowerPlugin;
use bevy::app::{PluginGroup, PluginGroupBuilder};

pub(crate) mod app_state;
pub(crate) mod glower;

pub(crate) struct SystemPlugin;

impl PluginGroup for SystemPlugin {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>() //
            .add(GlowerPlugin)
    }
}
