use gate::message::gate_header::GateHeader;
use hall::message::{GamePlayCardRequest, GamePlayCardResponse};
use shared_net::{op, NodeType};

use crate::private::logic::shared::update_user;
use crate::HallContext;

pub(crate) fn recv_game_play_card(context: &HallContext, request: GamePlayCardRequest, _: NodeType, header: GateHeader) -> Option<GamePlayCardResponse> {
    let mut games = context.games.write().unwrap();
    let success = update_user(&mut games, request.game_id, header.user, header.auth, op::Command::GamePlayCard, |user| user.state.play_cards(&request.picks));

    let response = GamePlayCardResponse {
        success,
    };
    Some(response)
}
