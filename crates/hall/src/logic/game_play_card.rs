use crate::logic::shared::update_user;
use crate::network::util::send_routed_message;
use crate::HallContext;
use gate::message::gate_header::GateHeader;
use hall::data::game::GamePhase;
use hall::message::{GamePlayCardRequest, GamePlayCardResponse, GameResolveCardsMessage};
use shared_net::types::NodeType;
use shared_net::{op, RoutedMessage, VSizedBuffer};
use tokio::sync::mpsc::UnboundedSender;

pub(crate) fn recv_game_play_card(context: HallContext, tx: UnboundedSender<RoutedMessage>, mut buf: VSizedBuffer) {
    let mut context = context.lock().unwrap();

    let gate = buf.pull::<NodeType>();
    let header = buf.pull::<GateHeader>();
    let request = buf.pull::<GamePlayCardRequest>();

    let success = update_user(
        &mut context.games,
        request.game_id,
        header.user,
        header.auth,
        op::Command::GamePlayCard, //
        |user| request.picks.iter().map(|(idx, target)| user.state.play_card(*idx, *target)).collect::<Vec<_>>(),
    );

    let response = GamePlayCardResponse {
        success,
    };

    let _ = send_routed_message(&response, gate, header.vagabond, &tx);

    if let Some(game) = context.games.get_mut(&request.game_id) {
        if game.all_users_last_command(op::Command::GamePlayCard) {
            let message = GameResolveCardsMessage {
                success: true,
            };
            game.set_phase(GamePhase::TurnEnd, op::Command::GameEndTurn);
            context.bx.broadcast(message);
        }
    }
}
