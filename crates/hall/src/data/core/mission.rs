use num_enum::{FromPrimitive, IntoPrimitive};
use serde::{Deserialize, Serialize};
use shared_net::Bufferable;
use shared_net::VSizedBuffer;

pub type MissionIdType = u8;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, FromPrimitive, IntoPrimitive)]
pub enum MissionNodeKind {
    #[num_enum(default)]
    AccessPoint,
    Backend,
    Control,
    Database,
    Engine,
    Frontend,
    Gateway,
    Hardware,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug, FromPrimitive, IntoPrimitive)]
pub enum MissionNodeState {
    #[num_enum(default)]
    Unknown,
    Known,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug, FromPrimitive, IntoPrimitive)]
pub enum MissionNodeLinkDir {
    #[num_enum(default)]
    North,
    East,
    South,
    West,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug, FromPrimitive, IntoPrimitive)]
pub enum MissionNodeLinkState {
    #[num_enum(default)]
    Closed,
    Open,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct MissionNodeLink {
    pub direction: MissionNodeLinkDir,
    pub target: MissionNodeIdType,
    pub state: MissionNodeLinkState,
}

#[derive(Bufferable, Default, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct MissionNodeContent {
    pub log: bool,
}

pub type MissionNodeIdType = u8;
