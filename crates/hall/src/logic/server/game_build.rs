use crate::network::broadcaster::Broadcaster;
use hall::data::game::{GamePhase, GameState};
use hall::message::GameStartGameMessage;
use shared_net::op;

pub(crate) fn handle_game_build(game: &mut GameState, bx: &mut Broadcaster) {
    if game.all_users_have_players() {
        game.set_phase(GamePhase::TurnStart, op::Command::GameStartTurn);
        let message = GameStartGameMessage {
            success: true,
        };
        bx.broadcast(message);
    }
}
