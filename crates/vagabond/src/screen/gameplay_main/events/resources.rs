use bevy::prelude::Event;

use hall::core::{AttributeKind, BuildValueType, ErgType};
use hall::message::GameResourcesMessage;

#[derive(Event)]
pub(crate) struct ResourcesTrigger {
    pub(crate) local_erg: [ErgType; 4],
    pub(crate) remote_erg: [ErgType; 4],
    pub(crate) remote_attr: [BuildValueType; 4],
    pub(crate) remote_kind: AttributeKind,
}

impl ResourcesTrigger {
    pub(crate) fn new(response: &GameResourcesMessage) -> Self {
        Self {
            local_erg: response.local_erg,
            remote_erg: response.remote_erg,
            remote_attr: response.remote_attr,
            remote_kind: response.remote_kind,
        }
    }
}
