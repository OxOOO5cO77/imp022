use crate::data::game::GameMission;
use shared_data::mission::{MissionNodeIdType, MissionNodeState};
use std::collections::HashMap;

#[derive(Default)]
pub struct PlayerMissionState {
    pub node: MissionNodeIdType,
    nodes: HashMap<MissionNodeIdType, MissionNodeState>,
}

impl PlayerMissionState {
    pub fn new(mission: &GameMission) -> Self {
        let mut this = Self {
            node: mission.node[0].id,
            nodes: HashMap::new(),
        };
        this.nodes.insert(this.node, MissionNodeState::Known);
        this
    }
}