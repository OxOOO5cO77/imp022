use hall_lib::core::Phase;
use hall_lib::message::GameStartGameMessage;

use crate::game::GameState;
use crate::network::broadcaster::Broadcaster;

pub(crate) fn handle_game_build(game: &mut GameState, bx: &mut Broadcaster) {
    if game.all_users_have_players() {
        game.set_phase(Phase::ChooseIntent);
        let message = GameStartGameMessage {
            success: true,
        };
        bx.broadcast(message);
    }
}
