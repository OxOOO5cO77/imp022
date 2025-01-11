use bevy::log::info;
use bevy::prelude::Commands;

use hall::core::MissionNodeKind;
use hall::view::GameMissionPlayerView;

use crate::manager::ScreenLayout;

pub(crate) struct Gateway {}

impl Gateway {
    pub(super) fn build_layout(_commands: &mut Commands, _layout: &ScreenLayout, _name: &str, _kind: MissionNodeKind) -> Self {
        Self {}
    }

    pub(crate) fn activate(&self, _commands: &mut Commands, _node: &GameMissionPlayerView) {
        info!("Activating gateway");
    }
}
