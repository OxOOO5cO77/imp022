use crate::logic::shared::update_user;
use crate::HallContext;
use gate::message::gate_header::GateHeader;
use hall::message::{GameStartTurnRequest, GameStartTurnResponse};
use shared_net::op;
use shared_net::types::NodeType;

pub(crate) fn recv_game_start_turn(context: &HallContext, request: GameStartTurnRequest, _: NodeType, header: GateHeader) -> Option<GameStartTurnResponse> {
    let mut games = context.games.write().unwrap();
    let success = update_user(&mut games, request.game_id, header.user, header.auth, op::Command::GameStartTurn, |_user| {
        /*TODO:*/
        true
    });

    let response = GameStartTurnResponse {
        success,
    };
    Some(response)
}
