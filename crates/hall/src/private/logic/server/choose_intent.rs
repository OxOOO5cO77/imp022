use std::collections::{HashMap, HashSet};

use hall::core::Phase;
use hall::message::GameRollMessage;

use crate::private::game::GameState;
use crate::private::logic::server::choose_intent::intents::IntentResult;
use crate::private::logic::server::update_mission::some_users_update_mission;
use crate::private::logic::server::update_state::all_users_update_state;
use crate::private::logic::server::update_tokens::some_users_update_tokens;
use crate::private::network::broadcaster::Broadcaster;

mod intents;

pub(crate) fn handle_choose_intent(game: &mut GameState, bx: &mut Broadcaster) {
    let mut intents = game
        .users
        .iter() //
        .filter(|(_, user)| game.mission.get_node(user.mission_state.current()).map(|node| node.kind.has_intent(user.state.intent)).unwrap_or(false))
        .map(|(id, user)| (*id, user.state.intent))
        .collect::<Vec<_>>();

    let tick = game.now();

    let mut node_changes = HashSet::new();
    let mut token_changes = HashMap::new();
    for (id, intent) in intents.drain(..) {
        if let Some(user) = game.users.get_mut(&id) {
            if let Some(result) = intents::process_intent(intent, &mut game.mission, user, &mut game.remotes, tick) {
                match result {
                    IntentResult::NodeChange => {
                        node_changes.insert(id);
                    }
                    IntentResult::TokenChange(messages) => {
                        token_changes.insert(id, messages);
                    }
                };
            }
        }
    }

    all_users_update_state(game, bx);
    some_users_update_mission(game, bx, node_changes);
    some_users_update_tokens(game, bx, token_changes);

    game.roll();

    let message = GameRollMessage {
        roll: game.erg_roll,
    };
    game.set_phase(Phase::ChooseAttr);
    bx.broadcast(message);
}
