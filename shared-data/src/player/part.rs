use serde::{Deserialize, Serialize};
use crate::player::build::Build;
use crate::player::category::Category;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct PlayerPart {
    pub seed: u64,
    pub values: [u8; 4],
    pub build: Vec<(Build, String)>,
    pub category: Vec<(Category, String)>,
}
