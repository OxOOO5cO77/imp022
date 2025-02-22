use gate_lib::message::gate_header::GateHeader;
use hall_lib::core::GameSubCommand;
use hall_lib::message::{GamePlayCardRequest, GamePlayCardResponse};
use shared_net::NodeType;

use crate::HallContext;
use crate::logic::shared::update_user;

pub(crate) fn recv_game_play_card(context: &HallContext, request: GamePlayCardRequest, _: NodeType, header: GateHeader) -> Option<GamePlayCardResponse> {
    let mut games = context.games.write().unwrap();
    let success = update_user(&mut games, request.game_id, header.user, header.auth, GameSubCommand::PlayCard, |user| user.state.play_cards(&request.picks));

    let response = GamePlayCardResponse {
        success,
    };
    Some(response)
}
