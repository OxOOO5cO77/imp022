use crate::logic::client::game_update_state::all_users_update_state;
use crate::network::broadcaster::Broadcaster;
use hall::data::game::{GamePhase, GameState, IdType};
use hall::message::{CardTarget, GameResolveCardsMessage};
use shared_net::op;

pub(crate) fn handle_play_card(game: &mut GameState, bx: &mut Broadcaster) {
    let mut all_played = Vec::new();
    for (user_id, user) in game.users.iter_mut() {
        for (card, target) in user.state.play.drain(..) {
            let played = match target {
                CardTarget::Local => (IdType::Local(*user_id), card.clone(), IdType::Local(*user_id)),
                CardTarget::Remote(_) => (IdType::Local(*user_id), card.clone(), IdType::Remote(user.remote.unwrap_or_default())), // todo: handle mission node -> remote conversion
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
