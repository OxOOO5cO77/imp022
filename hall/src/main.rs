use std::collections::{HashMap, VecDeque};
use std::collections::hash_map::Entry;
use std::env;
use std::sync::{Arc, Mutex};

use rand::prelude::*;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;

use hall::data::player::Player;
use hall::message::gamebuild::{GameBuildRequest, GameBuildResponse};
use hall::message::gamestart::{GameStartRequest, GameStartResponse};
use shared_net::{op, RoutedMessage, VClientMode, VSizedBuffer};
use shared_net::sizedbuffers::Bufferable;

use crate::manager::data_manager::DataManager;
use crate::manager::player_builder::PlayerBuilder;

pub(crate) mod manager;

struct Hall {
    games: HashMap<u128, Game>,
    data_manager: DataManager,
}

struct Game {
    users: HashMap<u128, User>,
}

struct User {
    auth: u128,
    parts: Vec<u64>,
    player: Option<Player>,
}

fn process_courtyard(context: Arc<Mutex<Hall>>, tx: UnboundedSender<RoutedMessage>, mut buf: VSizedBuffer) -> VClientMode {
    match buf.pull::<op::Command>() {
        op::Command::GameStart => c_gamestart(context, tx, buf),
        op::Command::GameBuild => c_gamebuild(context, tx, buf),
        op::Command::GameEnd => c_gameend(context, tx, buf),
        _ => {}
    }
    VClientMode::Continue
}

fn c_gamestart(context: Arc<Mutex<Hall>>, tx: UnboundedSender<RoutedMessage>, mut buf: VSizedBuffer) {
    let mut context = context.lock().unwrap();

    let gate = buf.pull::<u8>();
    let vagabond = buf.pull::<u8>();
    let user_hash = buf.pull::<u128>();
    let auth = buf.pull::<u128>();
    let request = buf.pull::<GameStartRequest>();

    let user = User {
        auth,
        parts: thread_rng().gen_iter().take(8).collect(),
        player: None,
    };

    let temp_builder = PlayerBuilder::new(&user.parts, &context.data_manager);

    let mut game_id = request.game_id;
    while game_id == 0 {
        let new_id = thread_rng().gen::<u128>();
        if !context.games.contains_key(&new_id) {
            game_id = new_id;
        }
    }
    let game = context.games.entry(game_id).or_insert_with(|| Game { users: Default::default() });

    game.users.insert(user_hash, user);

    println!("[Hall] [{:X}] Sending parts to G({})=>V({})", game_id, gate, vagabond);

    let parts = [
        temp_builder.access.convert_to_player_part(),
        temp_builder.breach.convert_to_player_part(),
        temp_builder.compute.convert_to_player_part(),
        temp_builder.disrupt.convert_to_player_part(),
        temp_builder.build.convert_to_player_part(),
        temp_builder.build_values.convert_to_player_part(),
        temp_builder.category.convert_to_player_part(),
        temp_builder.category_values.convert_to_player_part(),
    ];

    let route = op::Route::One(gate);
    let command = op::Command::GameStart;
    let response = GameStartResponse {
        game_id,
        parts,
    };

    let mut out = VSizedBuffer::new(route.size_in_buffer() + command.size_in_buffer() + vagabond.size_in_buffer() + response.size_in_buffer());


    out.push(&route);
    out.push(&command);
    out.push(&vagabond);
    out.push(&response);

    let _ = tx.send(RoutedMessage { route: op::Route::None, buf: out });
}

fn c_gamebuild(context: Arc<Mutex<Hall>>, tx: UnboundedSender<RoutedMessage>, mut buf: VSizedBuffer) {
    let context = context.lock().unwrap();

    let gate = buf.pull::<u8>();
    let vagabond = buf.pull::<u8>();
    let _user = buf.pull::<u128>();
    let _auth = buf.pull::<u128>();
    let request = buf.pull::<GameBuildRequest>();

    let builder = PlayerBuilder::new(&request.parts, &context.data_manager);
    let response = if let Some(player) = builder.create_player(&context.data_manager) {
        GameBuildResponse {
            seed: player.seed,
            deck: player.deck,
        }
    } else {
        GameBuildResponse {
            seed: 0,
            deck: VecDeque::default(),
        }
    };

    println!("[Hall] Sending build to G({})=>V({})", gate, vagabond);

    let route = op::Route::One(gate);
    let command = op::Command::GameBuild;

    let mut out = VSizedBuffer::new(route.size_in_buffer() + command.size_in_buffer() + vagabond.size_in_buffer() + response.size_in_buffer());

    out.push(&route);
    out.push(&command);
    out.push(&vagabond);
    out.push(&response);

    let _ = tx.send(RoutedMessage { route: op::Route::None, buf: out });
}


fn c_gameend(context: Arc<Mutex<Hall>>, tx: UnboundedSender<RoutedMessage>, mut buf: VSizedBuffer) {
    let mut context = context.lock().unwrap();

    let gate = buf.pull::<u8>();
    let vagabond = buf.pull::<u8>();
    let user_hash = buf.pull::<u128>();
    let auth = buf.pull::<u128>();
    let game_id = buf.pull::<u128>();

    if let Some(game) = context.games.get_mut(&game_id) {
        match game.users.entry(user_hash) {
            Entry::Occupied(user) if user.get().auth == auth => game.users.remove(&user_hash),
            _ => None,
        };

        if game.users.is_empty() {
            context.games.remove(&game_id);
        }
    }

    let mut out = VSizedBuffer::new(4);

    out.push(&op::Route::One(gate));
    out.push(&op::Command::GameEnd);
    out.push(&vagabond);

    let _ = tx.send(RoutedMessage { route: op::Route::None, buf: out });
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    println!("[Hall] START");

    let mut args = env::args();
    let _ = args.next(); // program name
    let iface_to_courtyard = args.next().unwrap_or("[::1]:12345".to_string());

    let (dummy_tx, dummy_rx) = mpsc::unbounded_channel();

    let context = Hall {
        games: HashMap::new(),
        data_manager: DataManager::new().expect("[Hall] Unable to initialize DataManager"),
    };

    let context = Arc::new(Mutex::new(context));

    let courtyard_client = shared_net::async_client(context, op::Flavor::Hall, dummy_tx, dummy_rx, iface_to_courtyard, process_courtyard);

    let result = courtyard_client.await;

    println!("[Hall] END");

    result
}
