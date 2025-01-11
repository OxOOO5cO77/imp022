use hall::core::AttributeValueType;
use shared_net::SeedType;

use crate::data::{VagabondBuild, VagabondDetail};

#[derive(Default, Clone)]
pub struct VagabondPart {
    pub seed: SeedType,
    pub values: [AttributeValueType; 4],
    pub build: [VagabondBuild; 4],
    pub detail: [VagabondDetail; 4],
}
