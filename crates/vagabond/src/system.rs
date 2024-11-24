use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::system::dragdrop::DragDropPlugin;

pub mod app_state;
pub(crate) mod dragdrop;
pub(crate) mod ui;

pub(crate) struct SystemPlugin;

impl PluginGroup for SystemPlugin {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(DragDropPlugin)
    }
}
