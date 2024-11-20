use serde::{Deserialize, Serialize};

use shared_data::detail::Detail;

type NumberType = u8;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct VagabondDetail {
    pub detail: Detail,
    pub number: NumberType,
    pub title: String,
}
