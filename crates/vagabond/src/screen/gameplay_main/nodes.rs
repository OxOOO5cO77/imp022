use bevy::ecs::system::IntoObserverSystem;
use bevy::prelude::*;

use hall::data::core::{MissionNodeIntent, MissionNodeKind, MissionNodeLinkDir};

use crate::manager::ScreenLayout;
use crate::screen::gameplay_main::components::MissionNodeDisplay;

mod access_point;
mod backend;
mod base_node;
mod control;
mod database;
mod engine;
mod frontend;
mod gateway;
mod hardware;

pub(crate) use base_node::BaseNode;

#[derive(Default)]
pub(super) enum MissionNodeAction {
    #[default]
    None,
    Link(Entity, MissionNodeLinkDir, Srgba),
}

impl From<&MissionNodeAction> for MissionNodeIntent {
    fn from(value: &MissionNodeAction) -> Self {
        match value {
            MissionNodeAction::None => MissionNodeIntent::None,
            MissionNodeAction::Link(_, dir, _) => MissionNodeIntent::Link(*dir),
        }
    }
}

pub(super) enum MissionNodeLayouts {
    Unknown,
    MissionNodeA(access_point::AccessPoint),
    MissionNodeB(backend::Backend),
    MissionNodeC(control::Control),
    MissionNodeD(database::Database),
    MissionNodeE(engine::Engine),
    MissionNodeF(frontend::Frontend),
    MissionNodeG(gateway::Gateway),
    MissionNodeH(hardware::Hardware),
}

impl MissionNodeLayouts {
    pub(super) fn build_layout(commands: &mut Commands, layout: &ScreenLayout, name: &str, kind: MissionNodeKind) -> Self {
        commands.entity(layout.entity(name)).insert(MissionNodeDisplay::new(kind));
        match kind {
            MissionNodeKind::Unknown => MissionNodeLayouts::Unknown,
            MissionNodeKind::AccessPoint => MissionNodeLayouts::MissionNodeA(access_point::AccessPoint::build_layout(commands, layout, name, kind)),
            MissionNodeKind::Backend => MissionNodeLayouts::MissionNodeB(backend::Backend::build_layout(commands, layout, name, kind)),
            MissionNodeKind::Control => MissionNodeLayouts::MissionNodeC(control::Control::build_layout(commands, layout, name, kind)),
            MissionNodeKind::Database => MissionNodeLayouts::MissionNodeD(database::Database::build_layout(commands, layout, name, kind)),
            MissionNodeKind::Engine => MissionNodeLayouts::MissionNodeE(engine::Engine::build_layout(commands, layout, name, kind)),
            MissionNodeKind::Frontend => MissionNodeLayouts::MissionNodeF(frontend::Frontend::build_layout(commands, layout, name, kind)),
            MissionNodeKind::Gateway => MissionNodeLayouts::MissionNodeG(gateway::Gateway::build_layout(commands, layout, name, kind)),
            MissionNodeKind::Hardware => MissionNodeLayouts::MissionNodeH(hardware::Hardware::build_layout(commands, layout, name, kind)),
        }
    }
}

#[derive(Component)]
pub(crate) struct NodeLocalObserver;

// local copy of observe to decorate observers with NodeLocalObserver to easily dispose before reactivation
fn local_observe<E: Event, B: Bundle, M>(observer: impl IntoObserverSystem<E, B, M>) -> impl EntityCommand {
    move |entity: Entity, world: &mut World| {
        if let Ok(mut world_entity) = world.get_entity_mut(entity) {
            world_entity.world_scope(|w| {
                w.spawn(Observer::new(observer).with_entity(entity)).insert(NodeLocalObserver);
            });
        }
    }
}
