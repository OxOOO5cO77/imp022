use hall::data::game::{GamePhase, GameState};
use hall::message::GameRollMessage;

use crate::logic::server::update_mission::some_users_update_mission;
use crate::logic::server::update_state::all_users_update_state;
use crate::network::broadcaster::Broadcaster;

pub(crate) fn handle_choose_intent(game: &mut GameState, bx: &mut Broadcaster) {
    let node_changes = game.process_intents();

    all_users_update_state(game, bx);
    some_users_update_mission(game, bx, node_changes);

    game.roll();

    let message = GameRollMessage {
        roll: game.erg_roll,
    };
    game.set_phase(GamePhase::ChooseAttr);
    bx.broadcast(message);
}
