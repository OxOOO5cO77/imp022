use crate::logic::client::game_update_state::all_users_update_state;
use crate::network::broadcaster::Broadcaster;
use hall::data::game::{GamePhase, GameState};
use hall::message::GameTickMessage;
use shared_net::op;

pub(crate) fn handle_end_turn(game: &mut GameState, bx: &mut Broadcaster) {
    let tick = game.tick();

    all_users_update_state(game, bx);

    let message = GameTickMessage {
        tick,
    };

    for remote in game.remotes.values_mut() {
        remote.reset();
    }

    game.set_phase(GamePhase::TurnStart, op::Command::GameStartTurn);
    bx.broadcast(message);
}
