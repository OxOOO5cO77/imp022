use std::sync::{Arc, Mutex};

use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::types::Uuid;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;
use tracing::{info, instrument};

use shared_net::{op, NodeType, PasswordType, RoutedMessage, UserIdType, VClientMode, VSizedBuffer};

struct Lookout {
    pool: PgPool,
}

#[allow(dead_code)]
#[derive(Debug)]
enum LookoutError {
    Environment(std::env::VarError),
    Database(sqlx::Error),
    Client(()),
}

#[tokio::main]
async fn main() -> Result<(), LookoutError> {
    tracing_subscriber::fmt::init();

    let mut args = std::env::args();
    let _ = args.next(); // program name
    let courtyard = args.next().unwrap_or("[::1]:12345".to_string());

    let db_connect = std::env::var("DB_CONNECT").map_err(LookoutError::Environment)?;

    lookout_main(courtyard, &db_connect).await
}

#[instrument]
async fn lookout_main(courtyard: String, database: &str) -> Result<(), LookoutError> {
    info!("START");

    let context = Arc::new(Mutex::new(Lookout {
        pool: PgPoolOptions::new().max_connections(16).connect(database).await.map_err(LookoutError::Database)?,
    }));

    let (dummy_tx, dummy_rx) = mpsc::unbounded_channel();

    let courtyard_client = shared_net::async_client(context, op::Flavor::Lookout, dummy_tx, dummy_rx, courtyard, process_courtyard);

    courtyard_client.await.map_err(LookoutError::Client)?;

    info!("END");

    Ok(())
}

fn process_courtyard(context: Arc<Mutex<Lookout>>, tx: UnboundedSender<RoutedMessage>, mut buf: VSizedBuffer) -> VClientMode {
    if buf.pull::<op::Command>() == op::Command::Authorize {
        c_authorize(context, tx, &mut buf)
    }
    VClientMode::Continue
}

#[derive(sqlx::FromRow)]
struct User {
    name: String,
    pass_uuid: Uuid,
}

fn c_authorize(context: Arc<Mutex<Lookout>>, tx: UnboundedSender<RoutedMessage>, buf: &mut VSizedBuffer) {
    let drawbridge = buf.pull::<NodeType>();
    let vagabond = buf.pull::<NodeType>();
    let user_hash = buf.pull::<UserIdType>();
    let pass_hash = buf.pull::<PasswordType>();

    let pool = context.lock().unwrap().pool.clone();

    let future = async move {
        let user_uuid = Uuid::from_u128(user_hash);
        let query_result = sqlx::query_as::<_, User>("SELECT * FROM users WHERE user_uuid = $1 LIMIT 1").bind(user_uuid).fetch_optional(&pool).await;
        match query_result {
            Ok(Some(user)) => {
                let pass = Uuid::as_u128(&user.pass_uuid);

                if pass_hash == pass {
                    info!(user_hash, "ALLOW: {}", user.name);
                    let auth = Uuid::new_v4().as_u128();

                    let mut out = VSizedBuffer::new(256);
                    out.push(&op::Route::Any(op::Flavor::Gate));
                    out.push(&op::Command::Authorize);
                    out.push(&drawbridge);
                    out.push(&vagabond);

                    out.push(&user_hash);
                    out.push(&auth);
                    out.push(&user.name);

                    let _ = tx.send(RoutedMessage {
                        route: op::Route::None,
                        buf: out,
                    });
                } else {
                    info!(user_hash, "DENY: {}", user.name);
                }
            }
            Ok(None) => {
                info!(user_hash, "UNKNOWN");
            }
            Err(err) => {
                info!(user_hash, "ERROR: {:?}", err);
            }
        }
    };
    tokio::spawn(future);
}

#[cfg(test)]
mod test {
    use fasthash::farm::fingerprint128;
    use uuid::Uuid;

    #[test]
    fn gen_uuids() {
        let user = Uuid::from_u128(fingerprint128("oxooo5co77"));
        let password = Uuid::from_u128(fingerprint128("password"));
        println!("User: {}    Password: {}", user, password);
    }
}
