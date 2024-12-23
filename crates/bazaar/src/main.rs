use std::sync::{Arc, Mutex};

use sqlx::postgres::{PgPool, PgPoolOptions};
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;
use tracing::{info, instrument};

use shared_net::op;
use shared_net::{RoutedMessage, VClientMode, VSizedBuffer};

struct Bazaar {
    _pool: PgPool,
}

#[allow(dead_code)]
#[derive(Debug)]
enum BazaarError {
    Environment(std::env::VarError),
    Database(sqlx::Error),
    Client(()),
}

#[tokio::main]
async fn main() -> Result<(), BazaarError> {
    tracing_subscriber::fmt::init();

    let mut args = std::env::args();
    let _ = args.next(); // program name
    let courtyard = args.next().unwrap_or("[::1]:12345".to_string());
    let db_connect = std::env::var("DB_CONNECT").map_err(BazaarError::Environment)?;

    bazaar_main(courtyard, &db_connect).await
}

#[instrument]
async fn bazaar_main(courtyard: String, database: &str) -> Result<(), BazaarError> {
    info!("START");

    let context = Arc::new(Mutex::new(Bazaar {
        _pool: PgPoolOptions::new().max_connections(16).connect(database).await.map_err(BazaarError::Database)?,
    }));

    let (dummy_tx, dummy_rx) = mpsc::unbounded_channel();

    let courtyard_client = shared_net::async_client(context, op::Flavor::Bazaar, dummy_tx, dummy_rx, courtyard, process_courtyard);

    courtyard_client.await.map_err(BazaarError::Client)?;

    info!("END");

    Ok(())
}

fn process_courtyard(_context: Arc<Mutex<Bazaar>>, _tx: UnboundedSender<RoutedMessage>, _buf: VSizedBuffer) -> VClientMode {
    VClientMode::Continue
}
