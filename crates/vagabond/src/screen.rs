use bevy::app::{PluginGroup, PluginGroupBuilder};

mod card_layout;
mod card_tooltip;
mod compose;
mod compose_init;
mod gameplay;
mod gameplay_init;
mod login;
mod splash;
mod util;

pub(crate) struct ScreenPlugins;

impl PluginGroup for ScreenPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>() //
            .add(splash::SplashPlugin)
            .add(login::LoginPlugin)
            .add(compose_init::ComposeInitPlugin)
            .add(compose::ComposePlugin)
            .add(gameplay_init::GameplayInitPlugin)
            .add(gameplay::GameplayPlugin)
    }
}
