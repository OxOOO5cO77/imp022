use serde::{Deserialize, Serialize};

pub type NumberType = u8;
pub type ValueType = u8;

pub type GeneralType = u8;
pub type SpecificType = u8;

#[derive(Default, Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum Detail {
    #[default] Any,
    Institution(GeneralType, SpecificType),
    Role(GeneralType, SpecificType),
    Location(GeneralType, SpecificType),
    Distro(GeneralType, SpecificType),
}
