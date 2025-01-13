use hall::core::Phase;
use hall::message::{CardTarget, GameResolveCardsMessage};

use crate::private::game::{GameState, IdType};
use crate::private::logic::server::update_state::all_users_update_state;
use crate::private::network::broadcaster::Broadcaster;

pub(crate) fn handle_play_card(game: &mut GameState, bx: &mut Broadcaster) {
    let mut all_played = Vec::new();
    for (user_id, user) in game.users.iter_mut() {
        let played = user.state.played_cards();
        for (card, target) in played {
            user.state.remove_erg(card.kind, card.cost);
            user.state.add_to_heap(card.clone());
            let played = match target {
                CardTarget::Local => (IdType::Local(*user_id), card, IdType::Local(*user_id)),
                CardTarget::Remote(node) => {
                    let remote = game.mission.get_node(node).or_else(|| game.mission.get_node(user.mission_state.current())).map(|n| n.remote).unwrap_or_default();
                    (IdType::Local(*user_id), card, IdType::Remote(remote))
                }
            };
            all_played.push(played);
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
