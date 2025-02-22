use gate_lib::message::gate_header::GateHeader;
use hall_lib::core::GameSubCommand;
use hall_lib::message::{GameChooseAttrRequest, GameChooseAttrResponse};
use shared_net::NodeType;

use crate::HallContext;
use crate::logic::shared::update_user;

pub(crate) fn recv_game_choose_attr(context: &HallContext, request: GameChooseAttrRequest, _: NodeType, header: GateHeader) -> Option<GameChooseAttrResponse> {
    let mut games = context.games.write().unwrap();
    let success = update_user(&mut games, request.game_id, header.user, header.auth, GameSubCommand::ChooseAttr, |user| {
        user.state.resolve_kind = Some(request.attr);
        true
    });

    let response = GameChooseAttrResponse {
        success,
    };
    Some(response)
}
