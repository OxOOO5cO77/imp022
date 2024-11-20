use crate::data::{VagabondBuild, VagabondDetail};
use shared_data::attribute;
use shared_net::types::SeedType;

#[derive(Default, Clone)]
pub struct VagabondPart {
    pub seed: SeedType,
    pub values: [attribute::AttributeValueType; 4],
    pub build: [VagabondBuild; 4],
    pub detail: [VagabondDetail; 4],
}
