use gate_lib::message::gate_header::GateHeader;
use hall_lib::message::{GameEndGameRequest, GameEndGameResponse};
use shared_net::NodeType;

use crate::HallContext;

pub(crate) fn recv_game_end_game(context: &HallContext, request: GameEndGameRequest, _: NodeType, _header: GateHeader) -> Option<GameEndGameResponse> {
    let mut games = context.games.write().unwrap();
    if let Some(game) = games.get_mut(&request.game_id)
        && game.is_empty()
    {
        games.remove(&request.game_id);
    }

    let response = GameEndGameResponse {
        success: true,
    };
    Some(response)
}
