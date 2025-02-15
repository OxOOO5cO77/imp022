use bevy::prelude::Event;

use hall_lib::view::GameMissionPlayerView;

#[derive(Event)]
pub(crate) struct MissionTrigger {
    pub(crate) mission: GameMissionPlayerView,
}

impl MissionTrigger {
    pub(crate) fn new(mission: GameMissionPlayerView) -> Self {
        Self {
            mission,
        }
    }
}
