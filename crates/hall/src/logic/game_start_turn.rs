use crate::logic::shared::update_user;
use crate::network::util::send_routed_message;
use crate::HallContext;
use gate::message::gate_header::GateHeader;
use hall::data::game::GamePhase;
use hall::message::{GameRollMessage, GameStartTurnRequest, GameStartTurnResponse};
use shared_net::types::NodeType;
use shared_net::{op, RoutedMessage, VSizedBuffer};
use tokio::sync::mpsc::UnboundedSender;

pub(crate) fn recv_game_start_turn(context: HallContext, tx: UnboundedSender<RoutedMessage>, mut buf: VSizedBuffer) {
    let mut context = context.lock().unwrap();

    let gate = buf.pull::<NodeType>();
    let header = buf.pull::<GateHeader>();
    let request = buf.pull::<GameStartTurnRequest>();

    let success = update_user(&mut context.games, request.game_id, header.user, header.auth, op::Command::GameStartTurn, |_user| {
        /*TODO:*/
        true
    });

    let response = GameStartTurnResponse {
        success,
    };

    let _ = send_routed_message(&response, gate, header.vagabond, &tx);

    if let Some(game) = context.games.get_mut(&request.game_id) {
        if game.all_users_last_command(op::Command::GameStartTurn) {
            game.roll();
            let message = GameRollMessage {
                roll: game.erg_roll,
            };
            game.set_phase(GamePhase::ChooseAttr, op::Command::GameChooseAttr);
            context.bx.broadcast(message);
        }
    }
}
