use hall::core::Phase;
use hall::message::GameStartGameMessage;

use crate::private::game::GameState;
use crate::private::network::broadcaster::Broadcaster;

pub(crate) fn handle_game_build(game: &mut GameState, bx: &mut Broadcaster) {
    if game.all_users_have_players() {
        game.set_phase(Phase::ChooseIntent);
        let message = GameStartGameMessage {
            success: true,
        };
        bx.broadcast(message);
    }
}
