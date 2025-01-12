use std::collections::HashMap;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;
use tracing::{info, warn};

use crate::AppState;
use shared_net::SeedType;
use warehouse::data::player_bio::PlayerBio;
use warehouse::rest::player::PlayerBioResponse;

fn process_player_bio(state: &AppState, params: &HashMap<String, String>) -> Option<PlayerBio> {
    let seed_string = params.get("seed")?;
    let seed = SeedType::from_str_radix(seed_string, 16).ok()?;
    let player_bio = state.bio_manager.generate_player_bio(seed);

    match &player_bio {
        Some(p) => info!("[Warehouse] /player/{seed_string} => {} ({},{},{})", p.name, p.birthplace.0, p.birthplace.1, p.birthplace.2),
        None => warn!("[Warehouse] /player/{seed_string} => INVALID"),
    }
    player_bio
}

pub(crate) async fn get(State(state): State<AppState>, Path(params): Path<HashMap<String, String>>) -> impl IntoResponse {
    let player_bio = process_player_bio(&state, &params);

    let response = PlayerBioResponse {
        player_bio,
    };

    Json(response)
}

#[cfg(test)]
mod test {
    #[tokio::test]
    #[cfg_attr(not(feature = "network-tests"), ignore)]
    async fn test_player() -> Result<(), httpc_test::Error> {
        let client = httpc_test::new_client("http://127.0.0.1:23235")?;

        client.do_get("/player/1234567890").await?.print().await?;

        Ok(())
    }
}
