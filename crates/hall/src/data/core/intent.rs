use crate::data::core::{MissionNodeKind, MissionNodeLinkDir};
use num_enum::{FromPrimitive, IntoPrimitive};
use serde::{Deserialize, Serialize};
use shared_net::{Bufferable, VSizedBuffer};

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, FromPrimitive, IntoPrimitive)]
#[cfg_attr(test, derive(Debug))]
pub enum AccessPointIntent {
    #[num_enum(default)]
    None,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, FromPrimitive, IntoPrimitive)]
#[cfg_attr(test, derive(Debug))]
pub enum BackendIntent {
    #[num_enum(default)]
    None,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, FromPrimitive, IntoPrimitive)]
#[cfg_attr(test, derive(Debug))]
pub enum ControlIntent {
    #[num_enum(default)]
    None,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, FromPrimitive, IntoPrimitive)]
#[cfg_attr(test, derive(Debug))]
pub enum DatabaseIntent {
    #[num_enum(default)]
    None,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, FromPrimitive, IntoPrimitive)]
#[cfg_attr(test, derive(Debug))]
pub enum EngineIntent {
    #[num_enum(default)]
    None,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, FromPrimitive, IntoPrimitive)]
#[cfg_attr(test, derive(Debug))]
pub enum FrontendIntent {
    #[num_enum(default)]
    None,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, FromPrimitive, IntoPrimitive)]
#[cfg_attr(test, derive(Debug))]
pub enum GatewayIntent {
    #[num_enum(default)]
    None,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, FromPrimitive, IntoPrimitive)]
#[cfg_attr(test, derive(Debug))]
pub enum HardwareIntent {
    #[num_enum(default)]
    None,
}

#[derive(Default, Clone, Copy)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum MissionNodeIntent {
    #[default]
    None,
    Link(MissionNodeLinkDir),
    AccessPoint(AccessPointIntent),
    Backend(BackendIntent),
    Control(ControlIntent),
    Database(DatabaseIntent),
    Engine(EngineIntent),
    Frontend(FrontendIntent),
    Gateway(GatewayIntent),
    Hardware(HardwareIntent),
}

impl MissionNodeKind {
    pub fn has_intent(&self, kind: MissionNodeIntent) -> bool {
        match kind {
            MissionNodeIntent::None => false,
            MissionNodeIntent::Link(_) => true,
            MissionNodeIntent::AccessPoint(_) => matches!(self, MissionNodeKind::AccessPoint),
            MissionNodeIntent::Backend(_) => matches!(self, MissionNodeKind::Backend),
            MissionNodeIntent::Control(_) => matches!(self, MissionNodeKind::Control),
            MissionNodeIntent::Database(_) => matches!(self, MissionNodeKind::Database),
            MissionNodeIntent::Engine(_) => matches!(self, MissionNodeKind::Engine),
            MissionNodeIntent::Frontend(_) => matches!(self, MissionNodeKind::Frontend),
            MissionNodeIntent::Gateway(_) => matches!(self, MissionNodeKind::Gateway),
            MissionNodeIntent::Hardware(_) => matches!(self, MissionNodeKind::Hardware),
        }
    }
}

impl MissionNodeIntent {
    pub fn is_for(&self, kind: MissionNodeKind) -> bool {
        match kind {
            MissionNodeKind::Unknown => matches!(self, MissionNodeIntent::Link(_)),
            MissionNodeKind::AccessPoint => matches!(self, MissionNodeIntent::AccessPoint(_)),
            MissionNodeKind::Backend => matches!(self, MissionNodeIntent::Backend(_)),
            MissionNodeKind::Control => matches!(self, MissionNodeIntent::Control(_)),
            MissionNodeKind::Database => matches!(self, MissionNodeIntent::Database(_)),
            MissionNodeKind::Engine => matches!(self, MissionNodeIntent::Engine(_)),
            MissionNodeKind::Frontend => matches!(self, MissionNodeIntent::Frontend(_)),
            MissionNodeKind::Gateway => matches!(self, MissionNodeIntent::Gateway(_)),
            MissionNodeKind::Hardware => matches!(self, MissionNodeIntent::Hardware(_)),
        }
    }
}

fn pack(intent: u8, action: u8) -> u16 {
    ((intent as u16) << 8) | action as u16
}

fn unpack(packed: u16) -> (u8, u8) {
    (((packed >> 8) & 0xFF) as u8, (packed & 0xFF) as u8)
}

impl Bufferable for MissionNodeIntent {
    fn push_into(&self, buf: &mut VSizedBuffer) {
        let packed = match self {
            MissionNodeIntent::None => 0u16,
            MissionNodeIntent::Link(dir) => pack(1, (*dir).into()),
            MissionNodeIntent::AccessPoint(action) => pack(2, (*action).into()),
            MissionNodeIntent::Backend(action) => pack(3, (*action).into()),
            MissionNodeIntent::Control(action) => pack(4, (*action).into()),
            MissionNodeIntent::Database(action) => pack(5, (*action).into()),
            MissionNodeIntent::Engine(action) => pack(6, (*action).into()),
            MissionNodeIntent::Frontend(action) => pack(7, (*action).into()),
            MissionNodeIntent::Gateway(action) => pack(8, (*action).into()),
            MissionNodeIntent::Hardware(action) => pack(9, (*action).into()),
        };
        packed.push_into(buf);
    }

    fn pull_from(buf: &mut VSizedBuffer) -> Self {
        let (intent, action) = unpack(u16::pull_from(buf));
        match intent {
            1 => MissionNodeIntent::Link(action.into()),
            2 => MissionNodeIntent::AccessPoint(action.into()),
            3 => MissionNodeIntent::Backend(action.into()),
            4 => MissionNodeIntent::Control(action.into()),
            5 => MissionNodeIntent::Database(action.into()),
            6 => MissionNodeIntent::Engine(action.into()),
            7 => MissionNodeIntent::Frontend(action.into()),
            8 => MissionNodeIntent::Gateway(action.into()),
            9 => MissionNodeIntent::Hardware(action.into()),
            _ => MissionNodeIntent::None,
        }
    }

    fn size_in_buffer(&self) -> usize {
        size_of::<u16>()
    }
}
