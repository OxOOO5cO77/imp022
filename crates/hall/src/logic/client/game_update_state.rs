use gate::message::gate_header::GateHeader;
use hall::data::game::GameState;
use hall::message::{GameUpdateStateRequest, GameUpdateStateResponse};
use shared_net::NodeType;

use crate::network::broadcaster::Broadcaster;
use crate::HallContext;

pub(crate) fn recv_game_update_state(context: &HallContext, request: GameUpdateStateRequest, _: NodeType, header: GateHeader) -> Option<GameUpdateStateResponse> {
    let games = context.games.read().unwrap();
    if let Some(game) = games.get(&request.game_id) {
        if let Some(user) = game.get_user_auth(header.user, header.auth) {
            if let Some(remote) = game.mission.remote_from_node(user.mission_state.node).and_then(|r| game.get_remote(r)) {
                let response = GameUpdateStateResponse::new(user, &remote.machine, &game.mission);
                return Some(response);
            }
        }
    }

    None
}

pub(crate) fn all_users_update_state(game: &mut GameState, bx: &mut Broadcaster) {
    for (id, user) in game.users.iter() {
        if let Some(remote_id) = game.mission.remote_from_node(user.mission_state.node) {
            if let Some(remote) = game.remotes.get(&remote_id) {
                let message = GameUpdateStateResponse::new(user, &remote.machine, &game.mission);
                bx.send_to_user(id, &message);
            }
        }
    }
}
