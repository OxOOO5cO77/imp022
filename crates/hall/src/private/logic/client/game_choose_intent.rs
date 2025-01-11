use gate::message::gate_header::GateHeader;
use hall::message::{GameChooseIntentRequest, GameChooseIntentResponse};
use shared_net::{op, NodeType};

use crate::private::logic::shared::update_user;
use crate::HallContext;

pub(crate) fn recv_game_choose_intent(context: &HallContext, request: GameChooseIntentRequest, _: NodeType, header: GateHeader) -> Option<GameChooseIntentResponse> {
    let mut games = context.games.write().unwrap();
    let success = update_user(&mut games, request.game_id, header.user, header.auth, op::Command::GameChooseIntent, |user| {
        user.state.intent = request.intent;
        true
    });

    let response = GameChooseIntentResponse {
        success,
    };
    Some(response)
}
