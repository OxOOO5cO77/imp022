use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use imp022_shared::player::build::Build;
use imp022_shared::player::category::Category;
use rand::prelude::StdRng;
use rand::SeedableRng;
use serde::{Deserialize, Serialize};

use crate::AppState;
use crate::data::data_manager::DataManager;

#[derive(Default, Serialize)]
struct Part {
    seed: u64,
    pub(crate) values: [u8; 4],
    pub(crate) build: Vec<(Build, String)>,
    pub(crate) category: Vec<(Category, String)>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct PartRequest {
    seeds: [u64; 8],
}

#[derive(Default, Serialize)]
struct PartResponse {
    parts: Vec<Part>,
}

fn make_part(dm: &DataManager, seed: u64) -> Part {
    let mut rng = StdRng::seed_from_u64(seed);

    Part {
        seed,
        values: DataManager::pick_values(&mut rng),
        build: if let Some(builds) = dm.pick_build(&mut rng) { builds.iter().map(|o| (o.build, o.title.clone())).collect() } else { Vec::default() },
        category: if let Some(categories) = dm.pick_category(&mut rng) { categories.iter().map(|o| (o.category, o.title.clone())).collect() } else { Vec::default() },
    }
}

pub(crate) async fn post(State(state): State<AppState>, request: Json<PartRequest>) -> impl IntoResponse {
    Json(PartResponse {
        parts: request.seeds.map(|o| make_part(&state.data_manager, o)).into_iter().collect(),
    })
}

#[cfg(test)]
mod test {
    use crate::route::part::PartRequest;

    #[tokio::test]
    async fn test_part() -> Result<(), httpc_test::Error> {
        let client = httpc_test::new_client("http://127.0.0.1:23235")?;

        let request = PartRequest {
            seeds: [1234567890, 1234567891, 1234567892, 1234567893, 1234567894, 1234567895, 1234567896, 1234567898]
        };

        let payload = serde_json::to_string(&request)?;
        client.do_post("/part", (payload, "application/json")).await?.print().await?;

        Ok(())
    }
}
