use std::sync::Arc;

use axum::Router;
use axum::routing::post;
use tokio::net::TcpListener;

use crate::data::data_manager::DataManager;

mod data;
mod route;

#[derive(Clone)]
struct AppState {
    data_manager: Arc<DataManager>,
}


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    println!("[Backend] START");

    let state = AppState {
        data_manager: Arc::new(DataManager::new()?)
    };

    println!("[Backend] State Initialized");

    let routes = Router::new()
        .route("/player", post(route::player::post))
        .route("/part", post(route::part::post))
        .with_state(state)
        ;

    let listener = TcpListener::bind("127.0.0.1:23235").await?;

    axum::serve(listener, routes).await?;

    println!("[Backend] END");
    Ok(())
}
