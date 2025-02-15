use hall_lib::core::AttributeArray;
use shared_net::SeedType;

use crate::data::{VagabondBuild, VagabondDetail};

#[derive(Default, Clone)]
pub struct VagabondPart {
    pub seed: SeedType,
    pub values: AttributeArray,
    pub build: [VagabondBuild; 4],
    pub detail: [VagabondDetail; 4],
}
