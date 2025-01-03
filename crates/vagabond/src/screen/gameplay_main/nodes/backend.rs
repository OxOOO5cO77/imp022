use crate::manager::ScreenLayout;
use bevy::log::info;
use bevy::prelude::Commands;
use hall::data::core::MissionNodeKind;
use hall::data::game::GameMissionNodePlayerView;

pub(crate) struct Backend {}

impl Backend {
    pub(super) fn build_layout(_commands: &mut Commands, _layout: &ScreenLayout, _name: &str, _kind: MissionNodeKind) -> Self {
        Self {}
    }

    pub(crate) fn activate(&self, _commands: &mut Commands, _node: &GameMissionNodePlayerView) {
        info!("Activating backend");
    }
}
