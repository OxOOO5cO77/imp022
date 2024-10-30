use bevy::prelude::Resource;

use warehouse::rest::player::PlayerBioResponse;

#[derive(Resource)]
pub(crate) struct WarehouseManager {
    address: String,
    client: reqwest::blocking::Client,
}

impl WarehouseManager {
    pub(crate) fn new(address: impl Into<String>) -> Self {
        WarehouseManager {
            address: address.into(),
            client: reqwest::blocking::Client::new(),
        }
    }

    pub fn fetch_player(&self, seed: u64) -> anyhow::Result<PlayerBioResponse> {
        let response =
            self.client.get(format!("{}/player/{:X}", self.address, seed))
                .send()?
                .json::<PlayerBioResponse>()?
            ;
        Ok(response)
    }
}
