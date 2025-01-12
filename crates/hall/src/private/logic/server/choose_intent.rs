use hall::core::Phase;
use hall::message::GameRollMessage;

use crate::private::game::GameState;
use crate::private::logic::server::update_mission::some_users_update_mission;
use crate::private::logic::server::update_state::all_users_update_state;
use crate::private::network::broadcaster::Broadcaster;

mod intents;

pub(crate) fn handle_choose_intent(game: &mut GameState, bx: &mut Broadcaster) {
    let intents = game
        .users
        .iter() //
        .filter(|(_, user)| game.mission.get_node(user.mission_state.current()).map(|node| node.kind.has_intent(user.state.intent)).unwrap_or(false))
        .map(|(id, user)| (*id, user.state.intent))
        .collect::<Vec<_>>();

    let mut node_changes = Vec::new();
    for (id, intent) in &intents {
        if let Some(user) = game.users.get_mut(id) {
            if intents::process_intent(*intent, user, &mut game.mission, &mut game.remotes) {
                node_changes.push(*id);
            }
        }
    }

    all_users_update_state(game, bx);
    some_users_update_mission(game, bx, node_changes);

    game.roll();

    let message = GameRollMessage {
        roll: game.erg_roll,
    };
    game.set_phase(Phase::ChooseAttr);
    bx.broadcast(message);
}
