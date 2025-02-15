use bevy::log::info;
use bevy::prelude::Commands;

use hall_lib::view::GameMissionPlayerView;

use crate::manager::ScreenLayout;

pub(crate) struct Control {}

impl Control {
    pub(super) fn build_layout(_commands: &mut Commands, _layout: &ScreenLayout, _name: &str) -> Self {
        Self {}
    }

    pub(crate) fn activate(&self, _commands: &mut Commands, _mission: &GameMissionPlayerView) {
        info!("Activating control");
    }
}
