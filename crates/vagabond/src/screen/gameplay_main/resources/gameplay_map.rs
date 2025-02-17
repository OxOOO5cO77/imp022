use std::collections::HashMap;

use bevy::prelude::{Entity, Resource};

use hall_lib::core::MissionNodeIdType;

pub(crate) const MAP_SIZE: usize = 8;

pub(crate) struct GameplayMapNode {
    pub(crate) entity: Entity,
    pub(crate) frame: Entity,
    pub(crate) text_id: Entity,
    pub(crate) text_kind: Entity,
}

impl GameplayMapNode {
    pub(crate) fn new(entity: Entity, frame: Entity, text_id: Entity, text_kind: Entity) -> Self {
        Self {
            entity,
            frame,
            text_id,
            text_kind,
        }
    }
}

#[derive(Resource)]
pub(crate) struct GameplayMap {
    pub(crate) main: Entity,
    pub(crate) node: HashMap<MissionNodeIdType, GameplayMapNode>,
    pub(crate) links: HashMap<(MissionNodeIdType, MissionNodeIdType), Entity>,
}

impl GameplayMap {
    pub(crate) fn new(main: Entity, node: HashMap<MissionNodeIdType, GameplayMapNode>, links: HashMap<(MissionNodeIdType, MissionNodeIdType), Entity>) -> Self {
        Self {
            main,
            node,
            links,
        }
    }
}
