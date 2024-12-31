use crate::screen::gameplay_main::nodes::{BaseNode, MissionNodeLayouts};
use bevy::prelude::Resource;
use hall::data::core::MissionNodeKind;
use std::collections::HashMap;

#[derive(Resource)]
pub(crate) struct NodeLayouts {
    pub(crate) base_node: BaseNode,
    pub(crate) layouts: HashMap<MissionNodeKind, MissionNodeLayouts>,
}
