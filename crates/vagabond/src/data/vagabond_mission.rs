use serde::{Deserialize, Serialize};
use shared_data::mission::{MissionIdType, MissionNodeIdType};

#[derive(Clone, Serialize, Deserialize)]
pub struct VagabondMissionNode {
    pub id: MissionNodeIdType,
    pub name: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct VagabondMissionObjective {}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct VagabondMission {
    pub id: MissionIdType,
    pub node: Vec<VagabondMissionNode>,
    pub objective: Vec<VagabondMissionObjective>,
}
