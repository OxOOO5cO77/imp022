use crate::logic::handle_phase_complete;
use crate::manager::data_manager::DataManager;
use crate::network::broadcaster::Broadcaster;
use crate::network::util::send_routed_message;
use gate::message::gate_header::GateHeader;
use hall::data::game::GameState;
use hall::message::{GameRequestMessage, GameResponseMessage};
use shared_net::types::{GameIdType, NodeType};
use shared_net::{op, RoutedMessage, VClientMode, VSizedBuffer};
use std::collections::HashMap;
use std::env;
use std::rc::Rc;
use std::sync::RwLock;
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::UnboundedSender;

mod logic;
mod manager;
mod network;

pub(crate) type HallContext = Rc<Hall>;
pub(crate) type HallGames = HashMap<GameIdType, GameState>;

struct Hall {
    games: RwLock<HallGames>,
    data_manager: RwLock<DataManager>,
    bx: RwLock<Broadcaster>,
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    println!("[Hall] START");

    let mut args = env::args();
    let _ = args.next(); // program name
    let iface_to_courtyard = args.next().unwrap_or("[::1]:12345".to_string());

    let (local_tx, local_rx) = mpsc::unbounded_channel();

    let context = Hall {
        games: RwLock::new(HashMap::new()),
        data_manager: RwLock::new(DataManager::new().expect("[Hall] Unable to initialize DataManager")),
        bx: RwLock::new(Broadcaster::new(local_tx.clone())),
    };
    let context = Rc::new(context);

    let courtyard_client = shared_net::async_client(context, op::Flavor::Hall, local_tx, local_rx, iface_to_courtyard, process_courtyard);

    let result = courtyard_client.await;

    println!("[Hall] END");

    result
}

fn process_courtyard(context: HallContext, tx: UnboundedSender<RoutedMessage>, mut buf: VSizedBuffer) -> VClientMode {
    let result = match buf.pull::<op::Command>() {
        op::Command::GameBuild => handle_recv(&context, tx, buf, logic::recv_game_build),
        op::Command::GameActivate => handle_recv(&context, tx, buf, logic::recv_game_activate),
        op::Command::GameStartTurn => handle_recv(&context, tx, buf, logic::recv_game_start_turn),
        op::Command::GameChooseAttr => handle_recv(&context, tx, buf, logic::recv_game_choose_attr),
        op::Command::GamePlayCard => handle_recv(&context, tx, buf, logic::recv_game_play_card),
        op::Command::GameEndTurn => handle_recv(&context, tx, buf, logic::recv_game_end_turn),
        op::Command::GameEndGame => handle_recv(&context, tx, buf, logic::recv_game_end_game),
        op::Command::GameUpdateState => handle_recv(&context, tx, buf, logic::recv_game_update_state),
        _ => return VClientMode::Continue,
    };

    if let Ok(game_id) = result {
        handle_phase_complete(context, game_id);
    }

    VClientMode::Continue
}

fn handle_recv<Request, Response>(context: &HallContext, tx: UnboundedSender<RoutedMessage>, mut buf: VSizedBuffer, handle_request: impl Fn(&HallContext, Request, NodeType, GateHeader) -> Option<Response>) -> Result<GameIdType, SendError<RoutedMessage>>
where
    Request: GameRequestMessage,
    Response: GameResponseMessage,
{
    let gate = buf.pull::<NodeType>();
    let header = buf.pull::<GateHeader>();
    let request = buf.pull::<Request>();

    let game_id = request.game_id();
    let vagabond = header.vagabond;

    let response = handle_request(context, request, gate, header);

    if let Some(response) = response {
        match send_routed_message(&response, gate, vagabond, &tx) {
            Ok(_) => Ok(game_id),
            Err(err) => Err(err),
        }
    } else {
        Ok(game_id)
    }
}
