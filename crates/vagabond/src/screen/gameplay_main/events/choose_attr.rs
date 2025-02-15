use bevy::prelude::Event;

use hall_lib::core::AttributeKind;

#[derive(Event)]
pub(crate) struct ChooseAttrTrigger {
    pub(crate) kind: Option<AttributeKind>,
}

impl ChooseAttrTrigger {
    pub(crate) fn new(kind: Option<AttributeKind>) -> Self {
        Self {
            kind,
        }
    }
}
