use bevy::prelude::{Commands, Entity, Query, Trigger, Visibility};

use crate::screen::gameplay_main::components::MissionNodeDisplay;
use crate::screen::gameplay_main::events::MissionTrigger;

pub(super) fn on_mission_ui_update(
    // bevy system
    event: Trigger<MissionTrigger>,
    mut commands: Commands,
    display_q: Query<(Entity, &MissionNodeDisplay)>,
) {
    for (entity, display) in &display_q {
        let visibility = if event.mission.node.kind == display.kind {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
        commands.entity(entity).insert(visibility);
    }
}
