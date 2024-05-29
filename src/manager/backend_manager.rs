use std::io::Error;

use bevy::prelude::Resource;
use imp022_shared::message::part::{PartRequest, PartResponse};
use imp022_shared::message::player::{PlayerRequest, PlayerResponse};

#[derive(Resource)]
pub(crate) struct BackendManager {
    client: reqwest::blocking::Client,
}

impl BackendManager {
    pub(crate) fn new() -> Result<Self, Error> {
        Ok(BackendManager {
            client: reqwest::blocking::Client::new(),
        })
    }

    pub fn fetch_player(&self, parts: [u64; 8]) -> anyhow::Result<PlayerResponse> {
        let request = PlayerRequest {
            access: parts[0],
            breach: parts[1],
            compute: parts[2],
            disrupt: parts[3],
            build: parts[4],
            build_values: parts[5],
            category: parts[6],
            category_values: parts[7],
        };

        Ok(
            self.client.post("http://127.0.0.1:23235/player")
                .json(&request)
                .send()?
                .json::<PlayerResponse>()?
        )
    }

    pub fn fetch_parts(&self, seeds: [u64; 8]) -> anyhow::Result<PartResponse> {
        let request = PartRequest {
            seeds,
        };

        Ok(
            self.client.post("http://127.0.0.1:23235/part")
                .json(&request)
                .send()?
                .json::<PartResponse>()?
        )
    }
}
