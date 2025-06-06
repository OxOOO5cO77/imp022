use std::sync::Arc;

use axum::Router;
use axum::routing::get;
use mimalloc::MiMalloc;
use tokio::net::TcpListener;
use tracing::{info, instrument};

use crate::manager::bio_manager::BioManager;

pub(crate) mod manager;
mod route;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[derive(Clone)]
struct AppState {
    bio_manager: Arc<BioManager>,
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    tracing_subscriber::fmt::init();

    warehouse_main().await
}

#[instrument]
async fn warehouse_main() -> Result<(), std::io::Error> {
    info!("START");

    let state = AppState {
        bio_manager: Arc::new(BioManager::new()?),
    };

    info!("State Initialized");

    let routes = Router::new() //
        .route("/player/{seed}", get(route::player::get))
        .route("/location/{seed}", get(route::location::get))
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:23235").await?;

    axum::serve(listener, routes).await?;

    info!("END");
    Ok(())
}
