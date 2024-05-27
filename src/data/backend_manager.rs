use std::io::Error;

use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

use crate::data::player::part::PlayerPart;
use crate::data::player::Player;

#[derive(Resource)]
pub(crate) struct BackendManager {
    client: reqwest::blocking::Client,
}

#[derive(Serialize)]
pub(crate) struct PlayerRequest {
    pub(crate) access: u64,
    pub(crate) breach: u64,
    pub(crate) compute: u64,
    pub(crate) disrupt: u64,
    pub(crate) build: u64,
    pub(crate) build_values: u64,
    pub(crate) category: u64,
    pub(crate) category_values: u64,
}

#[derive(Deserialize)]
pub(crate) struct PlayerResponse {
    pub(crate) player: Option<Player>,
}

#[derive(Serialize)]
pub(crate) struct PartRequest {
    seeds: [u64; 8],
}

#[derive(Default, Deserialize)]
pub(crate) struct PartResponse {
    pub(crate) parts: Vec<PlayerPart>,
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
        Ok(
            self.client.post("http://127.0.0.1:23235/part")
                .json(&PartRequest { seeds })
                .send()?
                .json::<PartResponse>()?
        )
    }
}
