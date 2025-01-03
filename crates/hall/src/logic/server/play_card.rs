use crate::logic::client::game_update_state::all_users_update_state;
use crate::network::broadcaster::Broadcaster;
use hall::data::game::{GamePhase, GameState, IdType};
use hall::message::{CardTarget, GameResolveCardsMessage};
use shared_net::op;

pub(crate) fn handle_play_card(game: &mut GameState, bx: &mut Broadcaster) {
    let mut all_played = Vec::new();
    for (user_id, user) in game.users.iter_mut() {
        let played = user.state.play.drain(..).collect::<Vec<_>>();
        for (card, target) in played {
            user.state.remove_erg(card.kind, card.cost);
            user.state.add_to_heap(card.clone());
            let played = match target {
                CardTarget::Local => (IdType::Local(*user_id), card, IdType::Local(*user_id)),
                CardTarget::Remote(node) => {
                    let remote = game.mission.remote_from_node(node).or_else(|| game.mission.remote_from_node(user.mission_state.node)).unwrap_or_default();
                    (IdType::Local(*user_id), card, IdType::Remote(remote))
                }
            };
            all_played.push(played);
        }
    }
    let _results = game.resolve_cards(all_played);

    game.set_phase(GamePhase::TurnEnd, op::Command::GameEndTurn);

    all_users_update_state(game, bx);

    let message = GameResolveCardsMessage {
        success: true,
    };
    bx.broadcast(message);
}
