use serde::{Deserialize, Serialize};

use hall_lib::core::{Build, BuildNumberType};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct VagabondBuild {
    pub build: Build,
    pub number: BuildNumberType,
    pub title: String,
}
