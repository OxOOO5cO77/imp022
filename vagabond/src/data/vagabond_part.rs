use shared_data::player::attribute;
use shared_data::types::SeedType;

use crate::data::vagabond_build::VagabondBuild;
use crate::data::vagabond_category::VagabondCategory;

#[derive(Default, Clone)]
pub struct VagabondPart {
    pub seed: SeedType,
    pub values: [attribute::ValueType; 4],
    pub build: [VagabondBuild; 4],
    pub category: [VagabondCategory; 4],
}
