use bevy::prelude::Event;

use hall::player::PlayerStatePlayerView;

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
