use bevy::app::{PluginGroup, PluginGroupBuilder};

mod compose;
mod splash;
mod gameplay;
mod login;
mod util;
mod card_layout;

pub(crate) struct ScreenPlugins;

impl PluginGroup for ScreenPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(splash::SplashPlugin)
            .add(login::LoginPlugin)
            .add(compose::ComposePlugin)
            .add(gameplay::GameplayPlugin)
    }
}
