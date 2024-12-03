use crate::manager::data_manager::DataManager;
use crate::network::broadcaster::Broadcaster;
use hall::data::game::GameState;
use shared_net::types::GameIdType;
use shared_net::{op, RoutedMessage, VClientMode, VSizedBuffer};
use std::collections::HashMap;
use std::env;
use std::rc::Rc;
use std::sync::Mutex;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;

mod logic;
mod manager;
mod network;

pub(crate) type HallContext = Rc<Mutex<Hall>>;
pub(crate) type HallGames = HashMap<GameIdType, GameState>;

struct Hall {
    games: HallGames,
    data_manager: DataManager,
    bx: Broadcaster,
}

impl Hall {
    fn split_borrow(&mut self) -> (&mut HallGames, &DataManager, &mut Broadcaster) {
        (&mut self.games, &self.data_manager, &mut self.bx)
    }
}


#[tokio::main]
async fn main() -> Result<(), ()> {
    println!("[Hall] START");

    let mut args = env::args();
    let _ = args.next(); // program name
    let iface_to_courtyard = args.next().unwrap_or("[::1]:12345".to_string());

    let (local_tx, local_rx) = mpsc::unbounded_channel();

    let context = Hall {
        games: HashMap::new(),
        data_manager: DataManager::new().expect("[Hall] Unable to initialize DataManager"),
        bx: Broadcaster::new(local_tx.clone()),
    };
    let context = Rc::new(Mutex::new(context));

    let courtyard_client = shared_net::async_client(context, op::Flavor::Hall, local_tx, local_rx, iface_to_courtyard, process_courtyard);

    let result = courtyard_client.await;

    println!("[Hall] END");

    result
}

fn process_courtyard(context: HallContext, tx: UnboundedSender<RoutedMessage>, mut buf: VSizedBuffer) -> VClientMode {
    match buf.pull::<op::Command>() {
        op::Command::GameActivate => logic::recv_game_activate(context, tx, buf),
        op::Command::GameBuild => logic::recv_game_build(context, tx, buf),
        op::Command::GameStartTurn => logic::recv_game_start_turn(context, tx, buf),
        op::Command::GameChooseAttr => logic::recv_game_choose_attr(context, tx, buf),
        op::Command::GamePlayCard => logic::recv_game_play_card(context, tx, buf),
        op::Command::GameEndTurn => logic::recv_game_end_turn(context, tx, buf),
        op::Command::GameEndGame => logic::recv_game_end(context, tx, buf),
        op::Command::GameUpdateState => logic::recv_game_update_state(context, tx, buf),
        _ => {}
    };
    VClientMode::Continue
}
