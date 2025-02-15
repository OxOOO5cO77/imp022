use num_enum::{FromPrimitive, IntoPrimitive};
use serde::{Deserialize, Serialize};

use shared_net::{Bufferable, SizedBuffer, SizedBufferError};

use crate::core::{AuthLevel, MissionNodeIdType};

#[repr(u8)]
#[derive(Default, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize, FromPrimitive, IntoPrimitive)]
#[cfg_attr(test, derive(Debug))]
pub enum MissionNodeKind {
    #[default]
    Unknown,
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
#[derive(Default, Clone, Copy, Serialize, Deserialize, FromPrimitive, IntoPrimitive)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum MissionNodeState {
    #[default]
    Unknown,
    Known,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, FromPrimitive, IntoPrimitive)]
#[cfg_attr(test, derive(Debug))]
pub enum MissionNodeLinkDir {
    #[num_enum(default)]
    North,
    East,
    South,
    West,
}

#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct MissionNodeLink {
    pub direction: MissionNodeLinkDir,
    pub target: MissionNodeIdType,
    pub min_level: AuthLevel,
}

#[derive(Bufferable, Default, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct MissionNodeContent {
    pub log: bool,
}
