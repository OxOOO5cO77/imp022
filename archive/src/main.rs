use std::env;
use std::sync::{Arc, Mutex};

use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::types::Uuid;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;

use shared_net::{op, RoutedMessage, VClientMode, VSizedBuffer};

struct Library {
    pool: PgPool,
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    println!("[Archive] START");

    let mut args = env::args();
    let _ = args.next(); // program name
    let iface_to_courtyard = args.next().unwrap_or("[::1]:12345".to_string());

    let db_connect = env::var("DB_CONNECT").expect("[Error] DB_CONNECT not set.");

    let context = Arc::new(Mutex::new(Library {
        pool: PgPoolOptions::new().max_connections(16).connect(&db_connect).await.unwrap()
    }));


    let (dummy_tx, dummy_rx) = mpsc::unbounded_channel();

    let courtyard_client = shared_net::async_client(context, op::Flavor::Archive, dummy_tx, dummy_rx, iface_to_courtyard, process_courtyard);

    let result = courtyard_client.await;
    
    println!("[Archive] END");
    
    result
}

fn process_courtyard(context: Arc<Mutex<Library>>, tx: UnboundedSender<RoutedMessage>, mut buf: VSizedBuffer) -> VClientMode {
    match buf.pull::<op::Command>() {
        op::Command::InvGen => c_invgen(context, tx, buf),
        op::Command::InvList => c_invlist(context, tx, buf),
        _ => {}
    }
    VClientMode::Continue
}

fn c_invgen(context: Arc<Mutex<Library>>, tx: UnboundedSender<RoutedMessage>, mut buf: VSizedBuffer) {
    let gate = buf.pull::<u8>();
    let vagabond = buf.pull::<u8>();
    let user_hash = buf.pull::<u128>();
    let _ob_type = buf.pull::<u8>();

    let pool = context.lock().unwrap().pool.clone();

    let future = async move {
        let user_uuid = Uuid::from_u128(user_hash);
        let object_uuid = Uuid::new_v4();
        let result = sqlx::query("INSERT INTO objects(user_uuid,ob_uuid) VALUES ( $1, $2 )")
            .bind(user_uuid)
            .bind(object_uuid)
            .execute(&pool).await.is_ok();
        if result {
            let mut out = VSizedBuffer::new(6 + 200 * 16);
            out.push(&op::Route::One(gate));
            out.push(&op::Command::InvList);
            out.push(&vagabond);

            out.push(&1_u16);
            out.push(&object_uuid.as_u128());

            let _ = tx.send(RoutedMessage { route: op::Route::None, buf: out });
        }
    };
    tokio::spawn(future);
}

#[derive(sqlx::FromRow)]
struct Object {
    ob_uuid: Uuid,
}

fn c_invlist(context: Arc<Mutex<Library>>, tx: UnboundedSender<RoutedMessage>, mut buf: VSizedBuffer) {
    let gate = buf.pull::<u8>();
    let vagabond = buf.pull::<u8>();
    let user_hash = buf.pull::<u128>();
    let _ob_type = buf.pull::<u8>();

    let pool = context.lock().unwrap().pool.clone();

    let future = async move {
        let user_uuid = Uuid::from_u128(user_hash);

        let query_result = sqlx::query_as::<_, Object>("SELECT (ob_uuid) FROM objects WHERE user_uuid = $1")
            .bind(user_uuid)
            .fetch_all(&pool).await;
        if let Ok(results) = query_result {
            let mut out = VSizedBuffer::new(6 + results.len() * 16);
            out.push(&op::Route::One(gate));
            out.push(&op::Command::InvList);
            out.push(&vagabond);
            out.push(&(results.len() as u16));
            for ob in results {
                out.push(&ob.ob_uuid.as_u128());
            }

            let _ = tx.send(RoutedMessage { route: op::Route::None, buf: out });
        }
    };
    tokio::spawn(future);
}
