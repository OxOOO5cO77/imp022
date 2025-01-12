use serde::{Deserialize, Serialize};

use crate::data::geo_location::GeoLocation;

#[derive(Serialize, Deserialize, Default)]
pub struct GeoLocationResponse {
    pub location: Option<GeoLocation>,
}
