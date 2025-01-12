use bevy::log::info;
use bevy::prelude::Commands;

use hall::view::GameMissionPlayerView;

use crate::manager::ScreenLayout;

pub(crate) struct Backend {}

impl Backend {
    pub(super) fn build_layout(_commands: &mut Commands, _layout: &ScreenLayout, _name: &str) -> Self {
        Self {}
    }

    pub(crate) fn activate(&self, _commands: &mut Commands, _mission: &GameMissionPlayerView) {
        info!("Activating backend");
    }
}
