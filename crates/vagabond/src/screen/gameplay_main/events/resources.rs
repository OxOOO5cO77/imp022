use bevy::prelude::Event;

use hall::core::{AttributeArray, AttributeKind, ErgArray};
use hall::message::GameResourcesMessage;

#[derive(Event)]
pub(crate) struct ResourcesTrigger {
    pub(crate) local_erg: ErgArray,
    pub(crate) remote_erg: ErgArray,
    pub(crate) remote_attr: AttributeArray,
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
