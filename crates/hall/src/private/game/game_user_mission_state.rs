use std::collections::HashMap;

use hall::core::{MissionNodeIdType, MissionNodeState};

#[derive(Default)]
pub struct GameUserMissionState {
    pub current_node: MissionNodeIdType,
    pub nodes: HashMap<MissionNodeIdType, MissionNodeState>,
}

impl GameUserMissionState {
    pub fn current(&self) -> MissionNodeIdType {
        self.current_node
    }

    pub fn known(&self) -> Vec<MissionNodeIdType> {
        self.nodes.iter().filter(|(_, state)| **state == MissionNodeState::Known).map(|(id, _)| *id).collect()
    }

    pub fn set_current(&mut self, node: MissionNodeIdType) -> bool {
        self.current_node = node;
        self.nodes.insert(node, MissionNodeState::Known).is_none_or(|prev| prev == MissionNodeState::Unknown)
    }
}
