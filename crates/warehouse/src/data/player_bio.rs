use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use shared_net::types::SeedType;

#[derive(Default, Serialize, Deserialize)]
pub struct PlayerBio {
    pub seed: SeedType,
    pub name: String,
    pub id: String,
    pub birthplace: (String, String, String),
    pub dob: NaiveDate,
}

impl PlayerBio {
    pub fn birthplace(&self) -> String {
        format!("{},{},{}", self.birthplace.0, self.birthplace.1, self.birthplace.2)
    }

    pub fn age(&self) -> u32 {
        NaiveDate::from_yo_opt(2049, 1).unwrap().years_since(self.dob).unwrap()
    }
}
