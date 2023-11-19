use bevy::app::{App, Plugin, PluginGroup, PluginGroupBuilder};
use crate::system::dragdrop::DragDropPlugin;

pub(crate) mod ui;
pub(crate) mod dragdrop;

struct SystemPlugins;

impl PluginGroup for SystemPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(DragDropPlugin)
    }
}

pub(crate) struct SystemPlugin;

impl Plugin for SystemPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(SystemPlugins)
        ;
    }
}
