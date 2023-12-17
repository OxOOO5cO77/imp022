use bevy::app::{App, Plugin, PluginGroup, PluginGroupBuilder};

use crate::screen::compose::ComposePlugin;
use crate::screen::splash::SplashPlugin;

mod compose;
mod splash;

pub(crate) struct ScreenPlugins;

impl PluginGroup for ScreenPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(SplashPlugin).add(ComposePlugin)
    }
}

pub(crate) struct MenusPlugin;

impl Plugin for MenusPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ScreenPlugins);
    }
}
