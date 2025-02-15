use serde::{Deserialize, Serialize};
use shared_net::SeedType;

#[derive(Default, Serialize, Deserialize)]
pub struct GeoLocation {
    pub seed: SeedType,
    pub location_part: (String, String, String),
}

impl GeoLocation {
    pub fn location(&self) -> String {
        format!("{}, {}, {}", self.location_part.0, self.location_part.1, self.location_part.2)
    }
}
