use std::env;
use std::sync::{Arc, Mutex};

use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::types::Uuid;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;

use shared_net::{VClientMode, VRoute, VRoutedMessage, VSizedBuffer};
use shared_net::op;

struct Lookout {
    pool: PgPool,
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let mut args = env::args();
    let _ = args.next(); // program name
    let iface_to_courtyard = args.next().unwrap_or("[::1]:12345".to_string());

    let db_connect = env::var("DB_CONNECT").unwrap_or("postgresql://lookout:lookout@[::1]:5432/lookout".to_string());

    let context = Arc::new(Mutex::new(Lookout {
        pool: PgPoolOptions::new().max_connections(16).connect(&db_connect).await.unwrap()
    }));

    let (dummy_tx, dummy_rx) = mpsc::unbounded_channel();

    let courtyard_client = shared_net::async_client(context, op::Flavor::Lookout, dummy_tx, dummy_rx, iface_to_courtyard, process_courtyard);

    courtyard_client.await
}

fn process_courtyard(context: Arc<Mutex<Lookout>>, tx: UnboundedSender<VRoutedMessage>, mut buf: VSizedBuffer) -> VClientMode {
    if buf.pull_command() == op::Command::Authorize { c_authorize(context, tx, &mut buf) }
    VClientMode::Continue
}

#[derive(sqlx::FromRow)]
struct User {
    name: String,
    pass_uuid: Uuid,
}

fn c_authorize(context: Arc<Mutex<Lookout>>, tx: UnboundedSender<VRoutedMessage>, buf: &mut VSizedBuffer) {
    let drawbridge = buf.pull_u8();
    let vagabond = buf.pull_u8();
    let user_hash = buf.pull_u128();
    let pass_hash = buf.pull_u128();

    let pool = context.lock().unwrap().pool.clone();

    let future = async move {
        let user_uuid = Uuid::from_u128(user_hash);
        let query_result = sqlx::query_as::<_, User>("SELECT * FROM users WHERE user_uuid = $1 LIMIT 1").bind(user_uuid).fetch_optional(&pool).await;
        if let Ok(Some(user)) = query_result {
            let pass = Uuid::as_u128(&user.pass_uuid);

            if pass_hash == pass {
                println!("[Lookout] Authenticated: {}", user.name);
                let guid = Uuid::new_v4().as_u128();

                let mut out = VSizedBuffer::new(256);
                out.push_route(op::Route::Any);
                out.push_flavor(op::Flavor::Gate);
                out.push_command(op::Command::Authorize);
                out.push_u8(&drawbridge);
                out.push_u8(&vagabond);

                out.push_u128(&guid);
                out.push_u128(&user_hash);
                out.push_string(&user.name);

                let _ = tx.send(VRoutedMessage { route: VRoute::None, buf: out });
            }
        } else {
            let err = query_result.err().unwrap();
            println!("[Lookout] ERROR: {:?}", err);
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
