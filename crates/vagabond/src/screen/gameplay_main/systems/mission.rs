use crate::screen::gameplay_main::components::MissionNodeDisplay;
use crate::screen::gameplay_main::events::{GamePhaseTrigger, MissionTrigger};
use crate::screen::gameplay_main::nodes::{BaseNode, MissionNodeAction, MissionNodeLayouts, NodeLocalObserver};
use crate::screen::gameplay_main::resources::{GameplayContext, NodeLayouts};
use crate::screen::gameplay_main::VagabondGamePhase;
use bevy::prelude::{Commands, Entity, Query, Res, ResMut, Trigger, Visibility, With};
use hall::data::game::{GameMissionNodePlayerView, GameMissionPlayerView};

fn cache_mission(mission: &GameMissionPlayerView) -> GameMissionPlayerView {
    GameMissionPlayerView {
        current_node: GameMissionNodePlayerView {
            id: mission.current_node.id,
            kind: mission.current_node.kind,
            state: mission.current_node.state,
            links: mission.current_node.links.clone(),
            content: mission.current_node.content.clone(),
            remote: mission.current_node.remote,
        },
        objective: mission.objective.clone(),
    }
}

pub(super) fn on_mission_ui_update(
    // bevy system
    event: Trigger<MissionTrigger>,
    mut commands: Commands,
    display_q: Query<(Entity, &MissionNodeDisplay)>,
    node_layouts: Res<NodeLayouts>,
    observer_q: Query<Entity, With<NodeLocalObserver>>,
    mut context: ResMut<GameplayContext>,
) {
    context.cached_mission = cache_mission(&event.mission);

    for observer in &observer_q {
        commands.entity(observer).despawn();
    }

    for (entity, display) in &display_q {
        let visibility = if event.mission.current_node.kind == display.kind {
            node_layouts.base_node.activate(&mut commands, &event.mission.current_node);
            if let Some(layout) = node_layouts.layouts.get(&display.kind) {
                match layout {
                    MissionNodeLayouts::MissionNodeA(access_point) => access_point.activate(&mut commands, &event.mission.current_node),
                    MissionNodeLayouts::MissionNodeB(backend) => backend.activate(&mut commands, &event.mission.current_node),
                    MissionNodeLayouts::MissionNodeC(control) => control.activate(&mut commands, &event.mission.current_node),
                    MissionNodeLayouts::MissionNodeD(database) => database.activate(&mut commands, &event.mission.current_node),
                    MissionNodeLayouts::MissionNodeE(engine) => engine.activate(&mut commands, &event.mission.current_node),
                    MissionNodeLayouts::MissionNodeF(frontend) => frontend.activate(&mut commands, &event.mission.current_node),
                    MissionNodeLayouts::MissionNodeG(gateway) => gateway.activate(&mut commands, &event.mission.current_node),
                    MissionNodeLayouts::MissionNodeH(hardware) => hardware.activate(&mut commands, &event.mission.current_node),
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
