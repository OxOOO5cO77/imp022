use std::env;
use std::sync::{Arc, Mutex};

use sqlx::postgres::{PgPool, PgPoolOptions};
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;

use shared_net::{VClientMode, VRoutedMessage, VSizedBuffer};
use shared_net::op;

struct Bazaar {
    _pool: PgPool,
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let mut args = env::args();
    let _ = args.next(); // program name
    let iface_to_courtyard = args.next().unwrap_or("[::1]:12345".to_string());

    let db_connect = env::var("DB_CONNECT").expect("[Error] DB_CONNECT not set.");

    let context = Arc::new(Mutex::new(Bazaar {
        _pool: PgPoolOptions::new().max_connections(16).connect(&db_connect).await.unwrap()
    }));


    let (dummy_tx, dummy_rx) = mpsc::unbounded_channel();

    let courtyard_client = shared_net::async_client(context, op::Flavor::Bazaar, dummy_tx, dummy_rx, iface_to_courtyard, process_courtyard);

    courtyard_client.await
}

fn process_courtyard(_context: Arc<Mutex<Bazaar>>, _tx: UnboundedSender<VRoutedMessage>, _buf: VSizedBuffer) -> VClientMode {
    VClientMode::Continue
}

