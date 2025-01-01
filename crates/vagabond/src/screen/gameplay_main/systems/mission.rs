use crate::screen::gameplay_main::components::MissionNodeDisplay;
use crate::screen::gameplay_main::events::{GamePhaseTrigger, MissionTrigger};
use crate::screen::gameplay_main::nodes::{BaseNode, MissionNodeLayouts, NodeLocalObserver};
use crate::screen::gameplay_main::resources::{GameplayContext, NodeLayouts};
use crate::screen::gameplay_main::VagabondGamePhase;
use bevy::prelude::{Commands, Entity, Query, Res, ResMut, Trigger, Visibility, With};

pub(super) fn on_mission_ui_update(
    // bevy system
    event: Trigger<MissionTrigger>,
    mut commands: Commands,
    display_q: Query<(Entity, &MissionNodeDisplay)>,
    node_layouts: Res<NodeLayouts>,
    observer_q: Query<Entity, With<NodeLocalObserver>>,
) {
    for observer in &observer_q {
        commands.entity(observer).despawn();
    }

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

pub(super) fn on_mission_phase_update(
    // bevy system
    event: Trigger<GamePhaseTrigger>,
    mut commands: Commands,
    mut context: ResMut<GameplayContext>,
) {
    match event.phase {
        VagabondGamePhase::Start => {}
        VagabondGamePhase::Pick => {
            BaseNode::deselect(&mut commands, &context);
            context.node_action = None;
        }
        VagabondGamePhase::Play => {}
        VagabondGamePhase::Draw => {}
        VagabondGamePhase::Wait(_) => {}
    }
}
