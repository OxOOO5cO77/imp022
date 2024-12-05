use crate::logic::shared::update_user;
use crate::HallContext;
use gate::message::gate_header::GateHeader;
use hall::message::{GameChooseAttrRequest, GameChooseAttrResponse};
use shared_net::op;
use shared_net::types::NodeType;

pub(crate) fn recv_game_choose_attr(context: &HallContext, request: GameChooseAttrRequest, _: NodeType, header: GateHeader) -> Option<GameChooseAttrResponse> {
    let mut games = context.games.write().unwrap();
    let success = update_user(&mut games, request.game_id, header.user, header.auth, op::Command::GameChooseAttr, |user| {
        user.state.resolve_kind = Some(request.attr.into());
        true
    });

    let response = GameChooseAttrResponse {
        success,
    };
    Some(response)
}
