use bevy::app::{App, Plugin, PluginGroup, PluginGroupBuilder};

use crate::screen::compose::ComposePlugin;
use crate::screen::splash::SplashPlugin;
use crate::screen::gameplay::GameplayPlugin;

mod compose;
mod splash;
mod gameplay;

pub(crate) struct ScreenPlugins;

impl PluginGroup for ScreenPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(SplashPlugin)
            .add(ComposePlugin)
            .add(GameplayPlugin)
    }
}

pub(crate) struct MenusPlugin;

impl Plugin for MenusPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ScreenPlugins);
    }
}
