use crate::private::game::{ExecutionResultKind, GameState, TargetIdType};
use crate::private::logic::server::update_state::all_users_update_state;
use crate::private::logic::server::update_tokens::some_users_update_tokens;
use crate::private::network::broadcaster::Broadcaster;
use hall::core::Phase;
use hall::message::GameTickMessage;
use std::collections::HashMap;

pub(crate) fn handle_end_turn(game: &mut GameState, bx: &mut Broadcaster) {
    let mut execution_results = game.tick();

    let mut machine_updates = HashMap::new();
    let mut token_updates = HashMap::new();
    for execution_result in execution_results.drain(..) {
        match execution_result.kind {
            ExecutionResultKind::TokenChange(update) => {
                let owner_entry = token_updates.entry(execution_result.owner).or_insert(Vec::new());
                owner_entry.push(update.clone());
                let target = match execution_result.target {
                    TargetIdType::User(user_id) if user_id != execution_result.owner => user_id,
                    _ => continue,
                };
                let target_entry = token_updates.entry(target).or_insert(Vec::new());
                target_entry.push(update);
            }
            ExecutionResultKind::ValueChange(value, amount) => {
                let machine_entry = machine_updates.entry(execution_result.owner).or_insert(Vec::new());
                machine_entry.push((value, amount))
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
