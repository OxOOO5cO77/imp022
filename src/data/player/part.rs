use serde::Deserialize;
use crate::data::player::build::Build;
use crate::data::player::category::Category;

#[derive(Default, Clone, Deserialize)]
pub(crate) struct PlayerPart {
    pub(crate) seed: u64,
    pub(crate) values: [u8; 4],
    pub(crate) build: Vec<(Build, String)>,
    pub(crate) category: Vec<(Category, String)>,
}
