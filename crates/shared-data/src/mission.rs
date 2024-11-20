use serde::{Deserialize, Serialize};

pub type MissionIdType = u8;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum MissionNodeKind {
    AccessPoint,
    Backend,
    Control,
    Database,
    Engine,
    Frontend,
    Gateway,
    Hardware,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum MissionNodeState {
    Unknown,
    Known,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum MissionNodeLinkDir {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum MissionNodeLinkState {
    Closed,
    Open,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MissionNodeLink {
    pub direction: MissionNodeLinkDir,
    pub target: MissionNodeIdType,
    pub state: MissionNodeLinkState,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MissionNodeContent {}

pub type MissionNodeIdType = u8;
