use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub(crate) enum AppState {
    #[default] Splash,
    LoginDrawbridge,
    LoginGate,
    Compose,
    Gameplay,
}
