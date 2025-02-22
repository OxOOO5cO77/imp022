use gate_lib::message::gate_header::GateHeader;
use hall_lib::core::GameSubCommand;
use hall_lib::message::{GameChooseIntentRequest, GameChooseIntentResponse};
use shared_net::NodeType;

use crate::HallContext;
use crate::logic::shared::update_user;

pub(crate) fn recv_game_choose_intent(context: &HallContext, request: GameChooseIntentRequest, _: NodeType, header: GateHeader) -> Option<GameChooseIntentResponse> {
    let mut games = context.games.write().unwrap();
    let success = update_user(&mut games, request.game_id, header.user, header.auth, GameSubCommand::ChooseIntent, |user| {
        user.state.intent = request.intent;
        true
    });

    let response = GameChooseIntentResponse {
        success,
    };
    Some(response)
}
