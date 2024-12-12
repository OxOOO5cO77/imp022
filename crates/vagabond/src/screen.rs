use bevy::app::{PluginGroup, PluginGroupBuilder};

mod card_layout;
mod compose;
mod gameplay;
mod login;
mod splash;

pub(crate) struct ScreenPlugins;

impl PluginGroup for ScreenPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(splash::SplashPlugin).add(login::LoginPlugin).add(compose::ComposePlugin).add(gameplay::GameplayPlugin)
    }
}
