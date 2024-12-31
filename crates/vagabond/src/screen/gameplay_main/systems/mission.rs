use crate::screen::gameplay_main::components::MissionNodeDisplay;
use crate::screen::gameplay_main::events::MissionTrigger;
use crate::screen::gameplay_main::nodes::MissionNodeLayouts;
use crate::screen::gameplay_main::resources::NodeLayouts;
use bevy::prelude::{Commands, Entity, Query, Res, Trigger, Visibility};

pub(super) fn on_mission_ui_update(
    // bevy system
    event: Trigger<MissionTrigger>,
    mut commands: Commands,
    display_q: Query<(Entity, &MissionNodeDisplay)>,
    node_layouts: Res<NodeLayouts>,
) {
    for (entity, display) in &display_q {
        let visibility = if event.mission.node.kind == display.kind {
            node_layouts.base_node.activate(&mut commands, &event.mission.node);
            if let Some(layout) = node_layouts.layouts.get(&display.kind) {
                match layout {
                    MissionNodeLayouts::MissionNodeA(access_point) => access_point.activate(&mut commands, &event.mission.node),
                    MissionNodeLayouts::MissionNodeB(backend) => backend.activate(&mut commands, &event.mission.node),
                }
            }
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
        commands.entity(entity).insert(visibility);
    }
}
