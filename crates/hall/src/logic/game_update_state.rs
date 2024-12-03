use crate::network::util::send_routed_message;
use crate::HallContext;
use gate::message::gate_header::GateHeader;
use hall::data::game::GameMachinePlayerView;
use hall::data::player::player_state::PlayerStatePlayerView;
use hall::message::{GameUpdateStateRequest, GameUpdateStateResponse};
use shared_net::types::NodeType;
use shared_net::{RoutedMessage, VSizedBuffer};
use tokio::sync::mpsc::UnboundedSender;

pub(crate) fn recv_game_update_state(context: HallContext, tx: UnboundedSender<RoutedMessage>, mut buf: VSizedBuffer) {
    let context = context.lock().unwrap();

    let gate = buf.pull::<NodeType>();
    let header = buf.pull::<GateHeader>();
    let request = buf.pull::<GameUpdateStateRequest>();

    if let Some(game) = context.games.get(&request.game_id) {
        if let Some(user) = game.get_user_auth(header.user, header.auth) {
            if let Some(remote) = game.get_remote(user.remote.unwrap_or_default()) {
                let response = GameUpdateStateResponse {
                    player_state: PlayerStatePlayerView::from(&user.state),
                    local_machine: GameMachinePlayerView::from(&user.machine),
                    remote_machine: GameMachinePlayerView::from(&remote.machine),
                };

                let _ = send_routed_message(&response, gate, header.vagabond, &tx);
            }
        }
    }
}
