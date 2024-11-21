use serde::{Deserialize, Serialize};

use shared_data::detail::{Detail, DetailNumberType};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct VagabondDetail {
    pub detail: Detail,
    pub number: DetailNumberType,
    pub title: String,
}
