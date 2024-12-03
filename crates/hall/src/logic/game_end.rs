use crate::network::util::send_routed_message;
use crate::HallContext;
use gate::message::gate_header::GateHeader;
use hall::message::GameEndGameResponse;
use shared_net::types::{GameIdType, NodeType};
use shared_net::{RoutedMessage, VSizedBuffer};
use tokio::sync::mpsc::UnboundedSender;

pub(crate) fn recv_game_end(context: HallContext, tx: UnboundedSender<RoutedMessage>, mut buf: VSizedBuffer) {
    let mut context = context.lock().unwrap();

    let gate = buf.pull::<NodeType>();
    let header = buf.pull::<GateHeader>();
    let game_id = buf.pull::<GameIdType>();

    if let Some(game) = context.games.get_mut(&game_id) {
        if game.is_empty() {
            context.games.remove(&game_id);
        }
    }

    let response = GameEndGameResponse {
        success: true,
    };

    let _ = send_routed_message(&response, gate, header.vagabond, &tx);
}
