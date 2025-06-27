use bevy::prelude::Resource;
use cached::{Cached, SizedCache};

use warehouse_lib::rest::location::GeoLocationResponse;
use warehouse_lib::rest::player::PlayerBioResponse;

#[derive(Resource)]
pub(crate) struct WarehouseManager {
    address: String,
    client: reqwest::blocking::Client,
    player: SizedCache<u64, PlayerBioResponse>,
    location: SizedCache<u64, GeoLocationResponse>,
}

impl WarehouseManager {
    pub(crate) fn new(address: impl Into<String>) -> Self {
        WarehouseManager {
            address: address.into(),
            client: reqwest::blocking::Client::new(),
            player: SizedCache::with_size(64),
            location: SizedCache::with_size(64),
        }
    }

    fn fetch<T: Default + for<'de> serde::Deserialize<'de>>(client: &reqwest::blocking::Client, address: &str, route: &str, seed: u64) -> T {
        client.get(format!("{address}/{route}/{seed:X}")).send().and_then(|response| response.json::<T>()).unwrap_or_default()
    }

    pub fn fetch_player(&mut self, seed: u64) -> anyhow::Result<&PlayerBioResponse> {
        let response = self.player.cache_get_or_set_with(seed, || Self::fetch(&self.client, &self.address, "player", seed));
        Ok(response)
    }

    pub fn fetch_location(&mut self, seed: u64) -> anyhow::Result<&GeoLocationResponse> {
        let response = self.location.cache_get_or_set_with(seed, || Self::fetch(&self.client, &self.address, "location", seed));
        Ok(response)
    }
}
