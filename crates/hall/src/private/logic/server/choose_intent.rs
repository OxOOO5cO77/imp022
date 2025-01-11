use hall::core::Phase;
use hall::message::GameRollMessage;

use crate::private::game::GameState;
use crate::private::logic::server::update_mission::some_users_update_mission;
use crate::private::logic::server::update_state::all_users_update_state;
use crate::private::network::broadcaster::Broadcaster;

pub(crate) fn handle_choose_intent(game: &mut GameState, bx: &mut Broadcaster) {
    let node_changes = game.process_intents();

    all_users_update_state(game, bx);
    some_users_update_mission(game, bx, node_changes);

    game.roll();

    let message = GameRollMessage {
        roll: game.erg_roll,
    };
    game.set_phase(Phase::ChooseAttr);
    bx.broadcast(message);
}
