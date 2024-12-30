use bevy::prelude::{Entity, Resource};

#[derive(Resource)]
pub(crate) struct DraggedPart {
    pub(crate) entity: Entity,
    pub(crate) active: bool,
}

impl DraggedPart {
    pub(crate) fn new(entity: Entity) -> Self {
        Self {
            entity,
            active: false,
        }
    }
}
