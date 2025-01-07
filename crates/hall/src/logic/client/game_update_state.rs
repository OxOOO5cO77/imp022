use gate::message::gate_header::GateHeader;
use hall::message::{GameUpdateStateRequest, GameUpdateStateResponse};
use shared_net::NodeType;

use crate::HallContext;

pub(crate) fn recv_game_update_state(context: &HallContext, request: GameUpdateStateRequest, _: NodeType, header: GateHeader) -> Option<GameUpdateStateResponse> {
    let games = context.games.read().unwrap();
    if let Some(game) = games.get(&request.game_id) {
        if let Some(user) = game.get_user_auth(header.user, header.auth) {
            if let Some(remote) = game.mission.get_node(user.mission_state.current()).and_then(|r| game.get_remote(r.remote)) {
                let response = GameUpdateStateResponse::new(user, &remote.machine, &game.mission);
                return Some(response);
            }
        }
    }

    None
}
