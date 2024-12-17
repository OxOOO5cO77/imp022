use std::collections::HashMap;

use crate::AppState;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;
use shared_net::types::SeedType;
use warehouse::rest::player::PlayerBioResponse;

pub(crate) async fn get(State(state): State<AppState>, Path(params): Path<HashMap<String, String>>) -> impl IntoResponse {
    let seed_string = params.get("seed").cloned().unwrap_or_default();
    let seed = SeedType::from_str_radix(&seed_string, 16).unwrap_or_default();
    let player_bio = state.bio_manager.generate_bio(seed);

    match &player_bio {
        Some(p) => println!("[Warehouse] /player/{seed_string} => {} ({},{},{})", p.name, p.birthplace.0, p.birthplace.1, p.birthplace.2),
        None => println!("[Warehouse] /player/{seed_string} => INVALID"),
    }

    Json(PlayerBioResponse {
        player_bio,
    })
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
