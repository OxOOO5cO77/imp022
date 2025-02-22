use std::sync::{Arc, Mutex};

use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::types::Uuid;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;
use tracing::{info, instrument};

use archive_lib::core::ArchiveSubCommand;
use gate_lib::message::gate_header::GateHeader;
use shared_net::op::SubCommandType;
use shared_net::{Bufferable, NodeType, RoutedMessage, SizedBuffer, SizedBufferError, VClientMode, op};

struct Archive {
    pool: PgPool,
}

#[allow(dead_code)]
#[derive(Debug)]
enum ArchiveError {
    Environment(std::env::VarError),
    Database(sqlx::Error),
    Client(()),
}

#[tokio::main]
async fn main() -> Result<(), ArchiveError> {
    tracing_subscriber::fmt::init();

    let mut args = std::env::args();
    let _ = args.next(); // program name
    let courtyard = args.next().unwrap_or("[::1]:12345".to_string());
    let db_connect = std::env::var("DB_CONNECT").map_err(ArchiveError::Environment)?;

    archive_main(courtyard, &db_connect).await
}

#[instrument]
async fn archive_main(courtyard: String, database: &str) -> Result<(), ArchiveError> {
    info!("START");

    let context = Arc::new(Mutex::new(Archive {
        pool: PgPoolOptions::new().max_connections(16).connect(database).await.map_err(ArchiveError::Database)?,
    }));

    let (dummy_tx, dummy_rx) = mpsc::unbounded_channel();

    let courtyard_client = shared_net::async_client(context, op::Flavor::Archive, dummy_tx, dummy_rx, courtyard, process_courtyard);

    courtyard_client.await.map_err(ArchiveError::Client)?;

    info!("END");

    Ok(())
}

fn process_courtyard(context: Arc<Mutex<Archive>>, tx: UnboundedSender<RoutedMessage>, mut buf: SizedBuffer) -> VClientMode {
    if let Ok(op::Command::Message(subcommand)) = buf.pull::<op::Command>() {
        let _result = match subcommand.into() {
            ArchiveSubCommand::InvGen => c_invgen(context, tx, buf),
            ArchiveSubCommand::InvList => c_invlist(context, tx, buf),
        };
    }

    VClientMode::Continue
}

fn c_invgen(context: Arc<Mutex<Archive>>, tx: UnboundedSender<RoutedMessage>, mut buf: SizedBuffer) -> Result<(), SizedBufferError> {
    let gate = buf.pull::<NodeType>()?;
    let header = buf.pull::<GateHeader>()?;
    let _ob_type = buf.pull::<u8>()?;

    let pool = context.lock().unwrap().pool.clone();

    let future = async move {
        let user_uuid = Uuid::from_u128(header.user);
        let object_uuid = Uuid::new_v4();
        let result = sqlx::query("INSERT INTO objects(user_uuid,ob_uuid) VALUES ( $1, $2 )").bind(user_uuid).bind(object_uuid).execute(&pool).await.is_ok();
        if result {
            if let Ok(out) = move || -> Result<SizedBuffer, SizedBufferError> {
                let route = op::Route::One(gate);
                let command = op::Command::Inventory(ArchiveSubCommand::InvList as SubCommandType);
                let results = vec![object_uuid.as_u128()];

                let mut out = SizedBuffer::new(route.size_in_buffer() + command.size_in_buffer() + header.vagabond.size_in_buffer() + results.size_in_buffer());
                out.push(&route)?;
                out.push(&command)?;
                out.push(&header.vagabond)?;
                out.push(&results)?;
                Ok(out)
            }() {
                let _ = tx.send(out.into());
            }
        }
    };
    tokio::spawn(future);
    Ok(())
}

#[derive(sqlx::FromRow)]
struct Object {
    ob_uuid: Uuid,
}

fn c_invlist(context: Arc<Mutex<Archive>>, tx: UnboundedSender<RoutedMessage>, mut buf: SizedBuffer) -> Result<(), SizedBufferError> {
    let gate = buf.pull::<NodeType>()?;
    let header = buf.pull::<GateHeader>()?;
    let _ob_type = buf.pull::<u8>()?;

    let pool = context.lock().unwrap().pool.clone();

    let future = async move {
        let user_uuid = Uuid::from_u128(header.user);

        let query_result = sqlx::query_as::<_, Object>("SELECT (ob_uuid) FROM objects WHERE user_uuid = $1").bind(user_uuid).fetch_all(&pool).await;
        if let Ok(results) = query_result {
            if let Ok(out) = move || -> Result<SizedBuffer, SizedBufferError> {
                let route = op::Route::One(gate);
                let command = op::Command::Inventory(ArchiveSubCommand::InvList as SubCommandType);
                let mapped_results = results.iter().map(|r| r.ob_uuid.as_u128()).collect::<Vec<_>>();

                let mut out = SizedBuffer::new(route.size_in_buffer() + command.size_in_buffer() + header.vagabond.size_in_buffer() + mapped_results.size_in_buffer());
                out.push(&op::Route::One(gate))?;
                out.push(&op::Command::Inventory(ArchiveSubCommand::InvList as SubCommandType))?;
                out.push(&header.vagabond)?;
                out.push(&mapped_results)?;
                Ok(out)
            }() {
                let _ = tx.send(out.into());
            }
        }
    };
    tokio::spawn(future);
    Ok(())
}
