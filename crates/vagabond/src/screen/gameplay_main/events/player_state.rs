use bevy::prelude::Event;

use hall_lib::view::GameUserStatePlayerView;

#[derive(Event)]
pub(crate) struct PlayerStateTrigger {
    pub(crate) state: GameUserStatePlayerView,
}

impl PlayerStateTrigger {
    pub(crate) fn new(state: GameUserStatePlayerView) -> Self {
        Self {
            state,
        }
    }
}
