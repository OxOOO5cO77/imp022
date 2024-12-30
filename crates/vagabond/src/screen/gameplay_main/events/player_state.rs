use bevy::prelude::Event;
use hall::data::player::PlayerStatePlayerView;

#[derive(Event)]
pub(crate) struct PlayerStateTrigger {
    pub(crate) state: PlayerStatePlayerView,
}

impl PlayerStateTrigger {
    pub(crate) fn new(state: PlayerStatePlayerView) -> Self {
        Self {
            state,
        }
    }
}
