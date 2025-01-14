use std::collections::HashSet;

use hall::core::MissionNodeIdType;

#[derive(Default)]
pub struct GameUserMissionState {
    pub current_node: MissionNodeIdType,
    pub known_nodes: HashSet<MissionNodeIdType>,
}

impl GameUserMissionState {
    pub fn current(&self) -> MissionNodeIdType {
        self.current_node
    }

    pub fn set_current(&mut self, node: MissionNodeIdType) -> bool {
        self.current_node = node;
        self.known_nodes.insert(node)
    }
}
