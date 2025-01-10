use crate::screen::gameplay_main::components::MissionNodeDisplay;
use crate::screen::gameplay_main::events::{GamePhaseTrigger, MissionTrigger};
use crate::screen::gameplay_main::nodes::{BaseNode, MissionNodeAction, MissionNodeLayouts, NodeLocalObserver};
use crate::screen::gameplay_main::resources::{GameplayContext, NodeLayouts};
use crate::screen::gameplay_main::VagabondGamePhase;
use bevy::prelude::{Commands, Entity, Query, Res, ResMut, Text2d, Trigger, Visibility, With};

pub(super) fn on_mission_ui_update(
    // bevy system
    event: Trigger<MissionTrigger>,
    mut commands: Commands,
    display_q: Query<(Entity, &MissionNodeDisplay)>,
    observer_q: Query<Entity, With<NodeLocalObserver>>,
    mut text_q: Query<&mut Text2d>,
    node_layouts: Res<NodeLayouts>,
    mut context: ResMut<GameplayContext>,
) {
    context.cached_mission = event.mission.clone();

    for observer in &observer_q {
        commands.entity(observer).despawn();
    }

    for (entity, display) in &display_q {
        let visibility = if event.mission.current().kind == display.kind {
            node_layouts.base_node.activate(&mut commands, &event.mission, &mut text_q);
            if let Some(layout) = node_layouts.layouts.get(&display.kind) {
                match layout {
                    MissionNodeLayouts::Unknown => {}
                    MissionNodeLayouts::MissionNodeA(access_point) => access_point.activate(&mut commands, &event.mission),
                    MissionNodeLayouts::MissionNodeB(backend) => backend.activate(&mut commands, &event.mission),
                    MissionNodeLayouts::MissionNodeC(control) => control.activate(&mut commands, &event.mission),
                    MissionNodeLayouts::MissionNodeD(database) => database.activate(&mut commands, &event.mission),
                    MissionNodeLayouts::MissionNodeE(engine) => engine.activate(&mut commands, &event.mission),
                    MissionNodeLayouts::MissionNodeF(frontend) => frontend.activate(&mut commands, &event.mission),
                    MissionNodeLayouts::MissionNodeG(gateway) => gateway.activate(&mut commands, &event.mission),
                    MissionNodeLayouts::MissionNodeH(hardware) => hardware.activate(&mut commands, &event.mission),
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
            context.node_action = MissionNodeAction::None;
        }
        VagabondGamePhase::Play => {}
        VagabondGamePhase::Draw => {}
        VagabondGamePhase::Wait(_) => {}
    }
}
