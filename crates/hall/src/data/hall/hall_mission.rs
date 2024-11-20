use serde::{Deserialize, Serialize};
use shared_data::mission::{MissionIdType, MissionNodeContent, MissionNodeIdType, MissionNodeKind, MissionNodeLink, MissionNodeState};

#[derive(Clone, Serialize, Deserialize)]
pub struct HallMissionNode {
    pub id: MissionNodeIdType,
    pub kind: MissionNodeKind,
    pub state: MissionNodeState,
    pub links: Vec<MissionNodeLink>,
    pub content: Vec<MissionNodeContent>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct HallMissionObjective {}

#[derive(Clone, Serialize, Deserialize)]
pub struct HallMission {
    pub id: MissionIdType,
    pub node: Vec<HallMissionNode>,
    pub objective: Vec<HallMissionObjective>,
}
