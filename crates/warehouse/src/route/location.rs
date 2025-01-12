use std::collections::HashMap;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;
use tracing::{info, warn};

use crate::AppState;
use shared_net::SeedType;
use warehouse::data::geo_location::GeoLocation;
use warehouse::rest::location::GeoLocationResponse;

fn process_geo_location(state: &AppState, params: &HashMap<String, String>) -> Option<GeoLocation> {
    let seed_string = params.get("seed")?;
    let seed = SeedType::from_str_radix(seed_string, 16).ok()?;
    let geo_location = state.bio_manager.generate_geo_location(seed);

    match &geo_location {
        Some(p) => info!("[Warehouse] /location/{seed_string} => {},{},{}", p.location_part.0, p.location_part.1, p.location_part.2),
        None => warn!("[Warehouse] /location/{seed_string} => INVALID"),
    }
    geo_location
}

pub(crate) async fn get(State(state): State<AppState>, Path(params): Path<HashMap<String, String>>) -> impl IntoResponse {
    let location = process_geo_location(&state, &params);

    let response = GeoLocationResponse {
        location,
    };

    Json(response)
}

#[cfg(test)]
mod test {
    #[tokio::test]
    #[cfg_attr(not(feature = "network-tests"), ignore)]
    async fn test_location() -> Result<(), httpc_test::Error> {
        let client = httpc_test::new_client("http://127.0.0.1:23235")?;

        client.do_get("/location/1234567890").await?.print().await?;

        Ok(())
    }
}
