use crate::network::broadcaster::Broadcaster;
use crate::HallContext;
use gate::message::gate_header::GateHeader;
use hall::data::game::{GameMachinePlayerView, GameState};
use hall::data::player::PlayerStatePlayerView;
use hall::message::{GameUpdateStateRequest, GameUpdateStateResponse};
use shared_net::types::NodeType;

pub(crate) fn recv_game_update_state(context: &HallContext, request: GameUpdateStateRequest, _: NodeType, header: GateHeader) -> Option<GameUpdateStateResponse> {
    let games = context.games.read().unwrap();
    if let Some(game) = games.get(&request.game_id) {
        if let Some(user) = game.get_user_auth(header.user, header.auth) {
            if let Some(remote) = game.get_remote(user.remote.unwrap_or_default()) {
                let response = GameUpdateStateResponse {
                    player_state: PlayerStatePlayerView::from(&user.state),
                    local_machine: GameMachinePlayerView::from(&user.machine),
                    remote_machine: GameMachinePlayerView::from(&remote.machine),
                };

                return Some(response);
            }
        }
    }

    None
}

pub(crate) fn all_users_update_state(game: &mut GameState, bx: &mut Broadcaster) {
    for (id, user) in game.users.iter() {
        if let Some(remote_id) = user.remote {
            if let Some(remote) = game.remotes.get(&remote_id) {
                let message = GameUpdateStateResponse {
                    player_state: PlayerStatePlayerView::from(&user.state),
                    local_machine: GameMachinePlayerView::from(&user.machine),
                    remote_machine: GameMachinePlayerView::from(&remote.machine),
                };
                bx.send_to_user(id, &message);
            }
        }
    }
}
