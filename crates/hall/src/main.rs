use std::collections::HashMap;
use std::rc::Rc;
use std::sync::RwLock;

use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::mpsc::error::SendError;
use tracing::{error, info, instrument};

use gate_lib::message::gate_header::GateHeader;
use hall_lib::core::GameSubCommand;
use hall_lib::message::{GameRequestMessage, GameResponseMessage};
use shared_net::{GameIdType, NodeType, RoutedMessage, SizedBuffer, SizedBufferError, VClientMode, op};

use game::GameState;
use logic::handle_phase_complete;
use manager::data_manager::DataManager;
use network::broadcaster::Broadcaster;
use network::util::send_routed_message;

pub(crate) mod game;
pub(crate) mod logic;
pub(crate) mod manager;
pub(crate) mod network;

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
    SizedBuffer(&'static str, SizedBufferError),
    Send(SendError<RoutedMessage>),
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

fn process_courtyard(context: HallContext, tx: UnboundedSender<RoutedMessage>, mut buf: SizedBuffer) -> VClientMode {
    let command = buf.pull::<op::Command>();

    if let Ok(op::Command::Game(subcommand)) = command {
        let result = match subcommand.into() {
            GameSubCommand::Build => handle_recv(&context, tx, buf, logic::recv_game_build),
            GameSubCommand::Activate => handle_recv(&context, tx, buf, logic::recv_game_activate),
            GameSubCommand::ChooseIntent => handle_recv(&context, tx, buf, logic::recv_game_choose_intent),
            GameSubCommand::ChooseAttr => handle_recv(&context, tx, buf, logic::recv_game_choose_attr),
            GameSubCommand::PlayCard => handle_recv(&context, tx, buf, logic::recv_game_play_card),
            GameSubCommand::EndTurn => handle_recv(&context, tx, buf, logic::recv_game_end_turn),
            GameSubCommand::EndGame => handle_recv(&context, tx, buf, logic::recv_game_end_game),
            GameSubCommand::UpdateState => handle_recv(&context, tx, buf, logic::recv_game_update_state),
            _ => return VClientMode::Continue,
        };

        match result {
            Ok(game_id) => handle_phase_complete(context, game_id),
            Err(e) => error!(?command, ?e),
        }
    }

    VClientMode::Continue
}

fn handle_recv<Request, Response>(context: &HallContext, tx: UnboundedSender<RoutedMessage>, mut buf: SizedBuffer, handle_request: impl Fn(&HallContext, Request, NodeType, GateHeader) -> Option<Response>) -> Result<GameIdType, HallError>
where
    Request: GameRequestMessage,
    Response: GameResponseMessage,
{
    let gate = buf.pull::<NodeType>().map_err(|e| HallError::SizedBuffer("gate", e))?;
    let header = buf.pull::<GateHeader>().map_err(|e| HallError::SizedBuffer("header", e))?;
    let request = buf.pull::<Request>().map_err(|e| HallError::SizedBuffer("request", e))?;

    let game_id = request.game_id();
    let vagabond = header.vagabond;

    let response = handle_request(context, request, gate, header);

    if let Some(response) = response {
        send_routed_message(&response, gate, vagabond, &tx)?;
    }

    Ok(game_id)
}
