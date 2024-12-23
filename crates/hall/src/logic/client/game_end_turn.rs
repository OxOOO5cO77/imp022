use gate::message::gate_header::GateHeader;
use hall::message::{GameEndTurnRequest, GameEndTurnResponse};
use shared_net::{op, NodeType};

use crate::logic::shared::update_user;
use crate::HallContext;

pub(crate) fn recv_game_end_turn(context: &HallContext, request: GameEndTurnRequest, _: NodeType, header: GateHeader) -> Option<GameEndTurnResponse> {
    let mut games = context.games.write().unwrap();
    let success = update_user(&mut games, request.game_id, header.user, header.auth, op::Command::GameEndTurn, |_user| {
        /*TODO:*/
        true
    });

    let response = GameEndTurnResponse {
        success,
    };
    Some(response)
}
