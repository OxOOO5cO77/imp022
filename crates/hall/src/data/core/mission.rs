use serde::{Deserialize, Serialize};

pub type MissionIdType = u8;

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
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

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum MissionNodeState {
    Unknown,
    Known,
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum MissionNodeLinkDir {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum MissionNodeLinkState {
    Closed,
    Open,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct MissionNodeLink {
    pub direction: MissionNodeLinkDir,
    pub target: MissionNodeIdType,
    pub state: MissionNodeLinkState,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct MissionNodeContent {}

pub type MissionNodeIdType = u8;
