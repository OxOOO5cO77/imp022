use hall::core::Phase;
use hall::message::{CardTarget, GameResolveCardsMessage};

use crate::private::game::{GameState, TargetIdType};
use crate::private::logic::server::update_state::all_users_update_state;
use crate::private::network::broadcaster::Broadcaster;

pub(crate) fn handle_play_card(game: &mut GameState, bx: &mut Broadcaster) {
    let mut all_played = Vec::new();
    for (user_id, user) in game.users.iter_mut() {
        let played = user.state.played_cards();
        for (card, target) in played {
            user.state.remove_erg(card.kind, card.cost);
            user.state.add_to_heap(card.clone());
            let remote_id = game.mission.get_node(user.mission_state.current()).map(|n| n.remote).unwrap_or_default();
            let target_id = match target {
                CardTarget::Local => TargetIdType::Local(*user_id),
                CardTarget::Remote(node) => {
                    let node_id = game.mission.get_node(node).map(|n| n.remote).unwrap_or(remote_id);
                    TargetIdType::Remote(node_id)
                }
            };
            all_played.push((*user_id, remote_id, card, target_id));
        }
    }
    let _results = game.resolve_cards(all_played);

    game.set_phase(Phase::TurnEnd);

    all_users_update_state(game, bx);

    let message = GameResolveCardsMessage {
        success: true,
    };
    bx.broadcast(message);
}
