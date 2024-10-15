use bevy::prelude::Resource;

use warehouse::rest::player::PlayerBioResponse;

#[derive(Resource)]
pub(crate) struct BackendManager {
    client: reqwest::blocking::Client,
}

impl BackendManager {
    pub(crate) fn new() -> Self {
        BackendManager {
            client: reqwest::blocking::Client::new(),
        }
    }

    pub fn fetch_player(&self, seed: u64) -> anyhow::Result<PlayerBioResponse> {
        let response =
            self.client.get(format!("http://127.0.0.1:23235/player/{seed:X}"))
                .send()?
                .json::<PlayerBioResponse>()?
            ;
        Ok(response)
    }
}
