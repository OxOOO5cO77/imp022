use bevy::prelude::*;

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
mod shared;

pub(crate) use base_node::BaseNode;
pub(crate) use shared::deselect_node_action;

use hall::core::*;

#[derive(Default)]
pub(crate) struct MissionNodeAction {
    pub(super) intent: Option<MissionNodeIntent>,
    pub(super) entity: Option<Entity>,
    pub(super) color: Srgba,
}

impl MissionNodeAction {
    pub(super) fn new(intent: Option<MissionNodeIntent>, entity: Option<Entity>, color: Srgba) -> Self {
        Self {
            intent,
            entity,
            color,
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
            MissionNodeKind::AccessPoint => MissionNodeLayouts::MissionNodeA(access_point::AccessPoint::build_layout(commands, layout, name)),
            MissionNodeKind::Backend => MissionNodeLayouts::MissionNodeB(backend::Backend::build_layout(commands, layout, name)),
            MissionNodeKind::Control => MissionNodeLayouts::MissionNodeC(control::Control::build_layout(commands, layout, name)),
            MissionNodeKind::Database => MissionNodeLayouts::MissionNodeD(database::Database::build_layout(commands, layout, name)),
            MissionNodeKind::Engine => MissionNodeLayouts::MissionNodeE(engine::Engine::build_layout(commands, layout, name)),
            MissionNodeKind::Frontend => MissionNodeLayouts::MissionNodeF(frontend::Frontend::build_layout(commands, layout, name)),
            MissionNodeKind::Gateway => MissionNodeLayouts::MissionNodeG(gateway::Gateway::build_layout(commands, layout, name)),
            MissionNodeKind::Hardware => MissionNodeLayouts::MissionNodeH(hardware::Hardware::build_layout(commands, layout, name)),
        }
    }
}
