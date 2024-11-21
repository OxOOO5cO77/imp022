use serde::{Deserialize, Serialize};

use shared_data::build::{Build, BuildNumberType};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct VagabondBuild {
    pub build: Build,
    pub number: BuildNumberType,
    pub title: String,
}
