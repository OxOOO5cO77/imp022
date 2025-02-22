use gate_lib::message::gate_header::GateHeader;
use hall_lib::core::GameSubCommand;
use hall_lib::message::{GameEndTurnRequest, GameEndTurnResponse};
use shared_net::NodeType;

use crate::HallContext;
use crate::logic::shared::update_user;

pub(crate) fn recv_game_end_turn(context: &HallContext, request: GameEndTurnRequest, _: NodeType, header: GateHeader) -> Option<GameEndTurnResponse> {
    let mut games = context.games.write().unwrap();
    let success = update_user(&mut games, request.game_id, header.user, header.auth, GameSubCommand::EndTurn, |_user| {
        /*TODO:*/
        true
    });

    let response = GameEndTurnResponse {
        success,
    };
    Some(response)
}
