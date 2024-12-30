use bevy::prelude::Component;
use vagabond::data::VagabondPart;

#[derive(Component, Default)]
pub(crate) struct PartHolder {
    pub(crate) part: Option<VagabondPart>,
}

impl PartHolder {
    pub(crate) fn new(part: VagabondPart) -> Self {
        Self {
            part: Some(part),
        }
    }
}
