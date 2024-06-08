use std::env;
use std::sync::{Arc, Mutex};

use rand::Rng;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;

use shared_net::{VClientMode, VRoute, VRoutedMessage, VSizedBuffer};
use shared_net::op;

struct Hall {
    games: Vec<Game>,
}

struct Game {
    id: u128,
    users: Vec<User>,
}

struct User {
    user_hash: u128,
    gate: u8,
    vagabond: u8,
    parts: Vec<u64>,
}

fn process_courtyard(context: Arc<Mutex<Hall>>, tx: UnboundedSender<VRoutedMessage>, mut buf: VSizedBuffer) -> VClientMode {
    match buf.pull_command() {
        op::Command::GameStart => c_gamestart(context, tx, buf),
        op::Command::GameEnd => c_gameend(context, tx, buf),
        _ => {}
    }
    VClientMode::Continue
}



fn generate_parts() -> Vec<u64> {
    rand::thread_rng().gen_iter().take(8).collect()
}

fn c_gamestart(context: Arc<Mutex<Hall>>, tx: UnboundedSender<VRoutedMessage>, mut buf: VSizedBuffer) {
    let mut context = context.lock().unwrap();

    let gate = buf.pull_u8();
    let vagabond = buf.pull_u8();
    let user_hash = buf.pull_u128();
    let _user_auth = buf.pull_u128();

    let user = User {
        user_hash,
        gate,
        vagabond,
        parts: generate_parts(),
    };

    let mut game = Game {
        id: rand::thread_rng().gen(),
        users: Vec::new(),
    };

    let part_count = user.parts.len();

    println!("[Hall] [{:X}] Sending {} parts to G({})=>V({})", game.id, user.parts.len(), gate, vagabond);

    let mut out = VSizedBuffer::new(5 + 16 + part_count * 8);

    out.push_route(op::Route::One);
    out.push_u8(&user.gate);
    out.push_command(op::Command::GameStart);
    out.push_u8(&user.vagabond);

    out.push_u128(&game.id);

    out.push_u8(&(part_count as u8));
    for part in user.parts.iter() {
        out.push_u64(part);
    }

    game.users.push(user);
    context.games.push(game);

    let _ = tx.send(VRoutedMessage { route: VRoute::None, buf: out });
}

fn c_gameend(context: Arc<Mutex<Hall>>, tx: UnboundedSender<VRoutedMessage>, mut buf: VSizedBuffer) {
    let mut context = context.lock().unwrap();

    let gate = buf.pull_u8();
    let vagabond = buf.pull_u8();
    let user_id = buf.pull_u128();
    let _user_auth = buf.pull_u128();
    let game_id = buf.pull_u128();

    if let Some(game_pos) = context.games.iter().position(|o| o.id == game_id) {
        let game = context.games.get_mut(game_pos).unwrap();
        game.users.retain(|o| o.user_hash != user_id);
        if game.users.is_empty() {
            context.games.swap_remove(game_pos);
        }
    }

    let mut out = VSizedBuffer::new(4);

    out.push_route(op::Route::One);
    out.push_u8(&gate);
    out.push_command(op::Command::GameEnd);
    out.push_u8(&vagabond);

    let _ = tx.send(VRoutedMessage { route: VRoute::None, buf: out });
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let mut args = env::args();
    let _ = args.next(); // program name
    let iface_to_courtyard = args.next().unwrap_or("[::1]:12345".to_string());

    let (dummy_tx, dummy_rx) = mpsc::unbounded_channel();

    let context = Hall {
        games: Vec::new()
    };

    let context = Arc::new(Mutex::new(context));

    let courtyard_client = shared_net::async_client(context, op::Flavor::Hall, dummy_tx, dummy_rx, iface_to_courtyard, process_courtyard);

    courtyard_client.await
}
