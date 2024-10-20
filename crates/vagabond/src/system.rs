use bevy::app::{App, Plugin, PluginGroup, PluginGroupBuilder};

use crate::system::dragdrop::DragDropPlugin;

pub(crate) mod dragdrop;
pub(crate) mod ui;
pub mod app_state;

struct SystemPlugins;

impl PluginGroup for SystemPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(DragDropPlugin)
    }
}

pub(crate) struct SystemPlugin;

impl Plugin for SystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SystemPlugins);
    }
}
