use std::collections::HashMap;

use bevy::prelude::Resource;

use hall::core::MissionNodeKind;

use crate::screen::gameplay_main::nodes::{BaseNode, MissionNodeLayouts};

#[derive(Resource)]
pub(crate) struct NodeLayouts {
    pub(crate) base_node: BaseNode,
    pub(crate) layouts: HashMap<MissionNodeKind, MissionNodeLayouts>,
}
