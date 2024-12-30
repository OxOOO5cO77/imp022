use bevy::app::{PluginGroup, PluginGroupBuilder};

mod compose_init;
mod compose_main;
mod gameplay_init;
mod gameplay_main;
mod login_drawbridge;
mod login_gate;
mod shared;
mod splash;

pub(crate) struct ScreenPlugins;

impl PluginGroup for ScreenPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>() //
            .add(splash::SplashPlugin)
            .add(login_drawbridge::LoginDrawbridgePlugin)
            .add(login_gate::LoginGatePlugin)
            .add(compose_init::ComposeInitPlugin)
            .add(compose_main::ComposeMainPlugin)
            .add(gameplay_init::GameplayInitPlugin)
            .add(gameplay_main::GameplayMainPlugin)
    }
}
