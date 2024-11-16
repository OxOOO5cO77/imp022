use shared_data::player::attribute;
use shared_data::types::SeedType;
use crate::data::{VagabondBuild, VagabondDetail};

#[derive(Default, Clone)]
pub struct VagabondPart {
    pub seed: SeedType,
    pub values: [attribute::AttributeValueType; 4],
    pub build: [VagabondBuild; 4],
    pub detail: [VagabondDetail; 4],
}
