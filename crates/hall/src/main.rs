use std::collections::HashMap;
use std::rc::Rc;
use std::sync::RwLock;

use tokio::sync::mpsc;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::UnboundedSender;
use tracing::{info, instrument};

use gate::message::gate_header::GateHeader;
use hall::message::{GameRequestMessage, GameResponseMessage};
use private::logic;
use shared_net::{op, GameIdType, NodeType, RoutedMessage, VClientMode, VSizedBuffer};

use crate::private::game::GameState;
use private::logic::handle_phase_complete;
use private::manager::data_manager::DataManager;
use private::network::broadcaster::Broadcaster;
use private::network::util::send_routed_message;

mod private;

pub(crate) type HallContext = Rc<Hall>;
pub(crate) type HallGames = HashMap<GameIdType, GameState>;

struct Hall {
    games: RwLock<HallGames>,
    data_manager: RwLock<DataManager>,
    bx: RwLock<Broadcaster>,
}

#[allow(dead_code)]
#[derive(Debug)]
enum HallError {
    Io(std::io::Error),
    Client(()),
}

#[tokio::main]
async fn main() -> Result<(), HallError> {
    tracing_subscriber::fmt::init();

    let mut args = std::env::args();
    let _ = args.next(); // program name
    let courtyard = args.next().unwrap_or("[::1]:12345".to_string());

    hall_main(courtyard).await
}

#[instrument]
async fn hall_main(courtyard: String) -> Result<(), HallError> {
    info!("START");

    let (local_tx, local_rx) = mpsc::unbounded_channel();

    let context = Hall {
        games: RwLock::new(HashMap::new()),
        data_manager: RwLock::new(DataManager::new().map_err(HallError::Io)?),
        bx: RwLock::new(Broadcaster::new(local_tx.clone())),
    };
    let context = Rc::new(context);

    shared_net::async_client(context, op::Flavor::Hall, local_tx, local_rx, courtyard, process_courtyard).await.map_err(HallError::Client)?;

    info!("END");

    Ok(())
}

fn process_courtyard(context: HallContext, tx: UnboundedSender<RoutedMessage>, mut buf: VSizedBuffer) -> VClientMode {
    let command = buf.pull::<op::Command>();

    let result = match command {
        op::Command::GameBuild => handle_recv(&context, tx, buf, logic::recv_game_build),
        op::Command::GameActivate => handle_recv(&context, tx, buf, logic::recv_game_activate),
        op::Command::GameChooseIntent => handle_recv(&context, tx, buf, logic::recv_game_choose_intent),
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
        send_routed_message(&response, gate, vagabond, &tx)?;
    }

    Ok(game_id)
}
