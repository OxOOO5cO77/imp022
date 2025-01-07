use hall::data::game::{GamePhase, GameState};
use hall::message::GameStartGameMessage;

use crate::network::broadcaster::Broadcaster;

pub(crate) fn handle_game_build(game: &mut GameState, bx: &mut Broadcaster) {
    if game.all_users_have_players() {
        game.set_phase(GamePhase::ChooseIntent);
        let message = GameStartGameMessage {
            success: true,
        };
        bx.broadcast(message);
    }
}
