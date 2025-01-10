use crate::data::core::{MissionNodeIdType, MissionNodeState};
use crate::data::game::GameMission;
use std::collections::HashMap;

#[derive(Default)]
pub struct PlayerMissionState {
    current_node: MissionNodeIdType,
    nodes: HashMap<MissionNodeIdType, MissionNodeState>,
}

impl PlayerMissionState {
    pub fn new(mission: &GameMission) -> Self {
        let mut this = Self {
            current_node: mission.node[0].id,
            nodes: HashMap::new(),
        };
        this.nodes.insert(this.current_node, MissionNodeState::Known);
        this
    }

    pub fn current(&self) -> MissionNodeIdType {
        self.current_node
    }

    pub fn known(&self) -> Vec<MissionNodeIdType> {
        self.nodes.iter().filter(|(_, state)| **state == MissionNodeState::Known).map(|(id, _)| *id).collect()
    }

    pub(crate) fn set_current(&mut self, node: MissionNodeIdType) -> bool {
        self.current_node = node;
        self.nodes.insert(node, MissionNodeState::Known).is_none_or(|prev| prev == MissionNodeState::Unknown)
    }
}
