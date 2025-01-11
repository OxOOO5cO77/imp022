use crate::private::game::GameState;
use crate::private::logic::server::update_state::all_users_update_state;
use crate::private::network::broadcaster::Broadcaster;
use hall::core::Phase;
use hall::message::GameTickMessage;

pub(crate) fn handle_end_turn(game: &mut GameState, bx: &mut Broadcaster) {
    let tick = game.tick();

    all_users_update_state(game, bx);

    let message = GameTickMessage {
        tick,
    };

    for remote in game.remotes.values_mut() {
        remote.reset();
    }

    game.set_phase(Phase::ChooseIntent);
    bx.broadcast(message);
}
