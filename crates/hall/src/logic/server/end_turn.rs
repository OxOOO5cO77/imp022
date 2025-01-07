use hall::data::game::{GamePhase, GameState};
use hall::message::GameTickMessage;

use crate::logic::server::update_state::all_users_update_state;
use crate::network::broadcaster::Broadcaster;

pub(crate) fn handle_end_turn(game: &mut GameState, bx: &mut Broadcaster) {
    let tick = game.tick();

    all_users_update_state(game, bx);

    let message = GameTickMessage {
        tick,
    };

    for remote in game.remotes.values_mut() {
        remote.reset();
    }

    game.set_phase(GamePhase::ChooseIntent);
    bx.broadcast(message);
}
