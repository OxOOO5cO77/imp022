use serde::{Deserialize, Serialize};

use shared_data::player::build::Build;

type NumberType = u8;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct VagabondBuild {
    pub build: Build,
    pub number: NumberType,
    pub title: String,
}
