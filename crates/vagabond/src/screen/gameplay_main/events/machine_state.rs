use bevy::prelude::Event;

use hall::view::GameMachinePlayerView;

#[derive(Event)]
pub(crate) struct MachineStateTrigger {
    pub(crate) local: GameMachinePlayerView,
    pub(crate) remote: GameMachinePlayerView,
}

impl MachineStateTrigger {
    pub(crate) fn new(local: GameMachinePlayerView, remote: GameMachinePlayerView) -> Self {
        Self {
            local,
            remote,
        }
    }
}
