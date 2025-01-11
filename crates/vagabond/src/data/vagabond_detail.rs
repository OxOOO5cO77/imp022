use serde::{Deserialize, Serialize};

use hall::core::{Detail, DetailNumberType};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct VagabondDetail {
    pub detail: Detail,
    pub number: DetailNumberType,
    pub title: String,
}
