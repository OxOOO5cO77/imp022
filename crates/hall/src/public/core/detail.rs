use serde::{Deserialize, Serialize};

use crate::core::{GeneralType, SpecificType};

#[derive(Default, Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum Detail {
    #[default]
    Any,
    Institution(GeneralType, SpecificType),
    Role(GeneralType, SpecificType),
    Location(GeneralType, SpecificType),
    Distro(GeneralType, SpecificType),
}
