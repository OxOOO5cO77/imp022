use bevy::prelude::Event;
use hall::data::core::AttributeKind;

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
