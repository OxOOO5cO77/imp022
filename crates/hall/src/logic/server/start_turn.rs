use crate::network::broadcaster::Broadcaster;
use hall::data::game::{GamePhase, GameState};
use hall::message::GameRollMessage;
use shared_net::op;

pub(crate) fn handle_start_turn(game: &mut GameState, bx: &mut Broadcaster) {
    game.roll();
    let message = GameRollMessage {
        roll: game.erg_roll,
    };
    game.set_phase(GamePhase::ChooseAttr, op::Command::GameChooseAttr);
    bx.broadcast(message);
}
