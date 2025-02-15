use std::collections::HashMap;

use hall_lib::core::Phase;
use hall_lib::message::GameTickMessage;

use crate::game::{ExecutionResultKind, GameState, TargetIdType};
use crate::logic::server::update_state::all_users_update_state;
use crate::logic::server::update_tokens::some_users_update_tokens;
use crate::network::broadcaster::Broadcaster;

pub(crate) fn handle_end_turn(game: &mut GameState, bx: &mut Broadcaster) {
    let mut execution_results = game.tick();

    let mut machine_updates = HashMap::new();
    let mut token_updates = HashMap::new();

    for mut execution_result in execution_results.drain(..) {
        for result in execution_result.results.drain(..) {
            match result {
                ExecutionResultKind::Token(update) => {
                    let owner_entry = token_updates.entry(execution_result.owner).or_insert(Vec::new());
                    owner_entry.push(update.clone());
                    let target = match execution_result.target {
                        TargetIdType::User(user_id) if user_id != execution_result.owner => user_id,
                        _ => continue,
                    };
                    let target_entry = token_updates.entry(target).or_insert(Vec::new());
                    target_entry.push(update);
                }
                ExecutionResultKind::Value(value, amount) => {
                    let machine_entry = machine_updates.entry(execution_result.owner).or_insert(Vec::new());
                    machine_entry.push((value, amount));
                }
                ExecutionResultKind::Deck => {
                    //
                }
            }
        }
    }

    game.set_phase(Phase::ChooseIntent);

    some_users_update_tokens(game, bx, token_updates);

    all_users_update_state(game, bx);

    let message = GameTickMessage {
        tick: game.now(),
    };
    bx.broadcast(message);
}
