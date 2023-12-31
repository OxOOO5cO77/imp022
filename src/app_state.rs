use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub(crate) enum AppState {
    SplashScreen,
    #[default] ComposeScreen,
}
