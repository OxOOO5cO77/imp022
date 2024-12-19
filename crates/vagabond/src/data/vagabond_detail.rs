use hall::data::core::{Detail, DetailNumberType};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct VagabondDetail {
    pub detail: Detail,
    pub number: DetailNumberType,
    pub title: String,
}
