use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;

use shared_data::rest::player::{PlayerRequest, PlayerResponse};

use crate::AppState;
use crate::data::player::player_builder::{PlayerBuilder, PlayerPartBuilder};

pub(crate) async fn post(State(state): State<AppState>, Json(request): Json<PlayerRequest>) -> impl IntoResponse {
    let player = PlayerBuilder {
        access: PlayerPartBuilder::new(&state.data_manager, request.access),
        breach: PlayerPartBuilder::new(&state.data_manager, request.breach),
        compute: PlayerPartBuilder::new(&state.data_manager, request.compute),
        disrupt: PlayerPartBuilder::new(&state.data_manager, request.disrupt),
        build: PlayerPartBuilder::new(&state.data_manager, request.build),
        build_values: PlayerPartBuilder::new(&state.data_manager, request.build_values),
        category: PlayerPartBuilder::new(&state.data_manager, request.category),
        category_values: PlayerPartBuilder::new(&state.data_manager, request.category_values),
    }.build_player(&state.data_manager);

    match &player {
        Some(p) => println!("[Backend] /player => {} ({},{},{})", p.name, p.birthplace.0, p.birthplace.1, p.birthplace.2),
        None => println!("[Backend] /player => INVALID"),
    }

    Json(PlayerResponse {
        player
    })
}

#[cfg(test)]
mod test {
    use crate::route::player::PlayerRequest;

    #[tokio::test]
    async fn test_player() -> Result<(), httpc_test::Error> {
        let client = httpc_test::new_client("http://127.0.0.1:23235")?;

        let request = PlayerRequest {
            access: 1234567890,
            breach: 1234567891,
            compute: 1234567892,
            disrupt: 1234567893,
            build: 1234567894,
            build_values: 1234567895,
            category: 1234567896,
            category_values: 1234567897,
        };

        let payload = serde_json::to_string(&request)?;
        client.do_post("/player", (payload, "application/json")).await?.print().await?;

        Ok(())
    }
}
