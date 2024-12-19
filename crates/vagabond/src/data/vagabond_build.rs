use hall::data::core::{Build, BuildNumberType};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct VagabondBuild {
    pub build: Build,
    pub number: BuildNumberType,
    pub title: String,
}
