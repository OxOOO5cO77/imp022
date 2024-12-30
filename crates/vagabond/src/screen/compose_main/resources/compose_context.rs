use bevy::prelude::Resource;

use hall::data::core::Attributes;
use vagabond::data::VagabondCard;

#[derive(Default, PartialEq)]
pub(crate) enum ComposeState {
    #[default]
    Build,
    Ready,
    Committed,
}

#[derive(Resource, Default)]
pub(crate) struct ComposeContext {
    pub(crate) state: ComposeState,
    pub(crate) deck: Vec<VagabondCard>,
    pub(crate) attributes: Attributes,
}
