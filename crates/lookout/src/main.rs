use std::env;
use std::sync::{Arc, Mutex};

use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::types::Uuid;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;
use shared_data::types::{PasswordType, NodeType, UserIdType};

use shared_net::{op, RoutedMessage, VClientMode, VSizedBuffer};

struct Lookout {
    pool: PgPool,
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    println!("[Lookout] START");

    let mut args = env::args();
    let _ = args.next(); // program name
    let iface_to_courtyard = args.next().unwrap_or("[::1]:12345".to_string());

    let db_connect = env::var("DB_CONNECT").unwrap_or("postgresql://lookout:lookout@[::1]:5432/lookout".to_string());

    let context = Arc::new(Mutex::new(Lookout {
        pool: PgPoolOptions::new().max_connections(16).connect(&db_connect).await.unwrap()
    }));

    let (dummy_tx, dummy_rx) = mpsc::unbounded_channel();

    let courtyard_client = shared_net::async_client(context, op::Flavor::Lookout, dummy_tx, dummy_rx, iface_to_courtyard, process_courtyard);

    let result = courtyard_client.await;

    println!("[Lookout] START");

    result
}

fn process_courtyard(context: Arc<Mutex<Lookout>>, tx: UnboundedSender<RoutedMessage>, mut buf: VSizedBuffer) -> VClientMode {
    if buf.pull::<op::Command>() == op::Command::Authorize { c_authorize(context, tx, &mut buf) }
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
                    println!("[Lookout] ALLOW: {}", user.name);
                    let auth = Uuid::new_v4().as_u128();

                    let mut out = VSizedBuffer::new(256);
                    out.push(&op::Route::Any(op::Flavor::Gate));
                    out.push(&op::Command::Authorize);
                    out.push(&drawbridge);
                    out.push(&vagabond);

                    out.push(&user_hash);
                    out.push(&auth);
                    out.push(&user.name);

                    let _ = tx.send(RoutedMessage { route: op::Route::None, buf: out });
                } else {
                    println!("[Lookout] DENY: {}", user.name);
                }
            }
            Ok(None) => {
                println!("[Lookout] UNKNOWN: {}", user_hash);
            }
            Err(err) => {
                println!("[Lookout] ERROR: {:?}", err);
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
