use std::sync::Arc;

use axum::Router;
use axum::routing::get;
use tokio::net::TcpListener;

use crate::manager::bio_manager::BioManager;

pub(crate) mod manager;
mod route;

#[derive(Clone)]
struct AppState {
    bio_manager: Arc<BioManager>,
}


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    println!("[Warehouse] START");

    let state = AppState {
        bio_manager: Arc::new(BioManager::new()?)
    };

    println!("[Warehouse] State Initialized");

    let routes = Router::new()
        .route("/player/:seed", get(route::player::get))
        .with_state(state)
        ;

    let listener = TcpListener::bind("127.0.0.1:23235").await?;

    axum::serve(listener, routes).await?;

    println!("[Warehouse] END");
    Ok(())
}
