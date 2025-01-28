use hall::core::{Phase, PickedCardTarget};
use hall::message::GameResolveCardsMessage;

use crate::private::game::{CardResolve, GameState, TargetIdType};
use crate::private::logic::server::update_state::all_users_update_state;
use crate::private::network::broadcaster::Broadcaster;

pub(crate) fn handle_play_card(game: &mut GameState, bx: &mut Broadcaster) {
    let mut all_played = Vec::new();
    for (user_id, user) in game.users.iter_mut() {
        let played = user.state.played_cards();
        for (card, target) in played {
            user.state.remove_erg(card.kind, card.cost);
            user.state.add_to_heap(card.clone());
            let current_node = game.mission.get_node(user.mission_state.current());
            let remote_id = current_node.map(|n| n.remote).unwrap_or_default();
            let target_id = match target {
                PickedCardTarget::None => TargetIdType::None,
                PickedCardTarget::MachineLocal => TargetIdType::User(*user_id),
                PickedCardTarget::MachineRemote => TargetIdType::Remote(remote_id),
                PickedCardTarget::Actor(index) => {
                    if let Some(actor) = current_node.and_then(|n| n.actors.get(index as usize)) {
                        TargetIdType::Actor(*actor)
                    } else {
                        TargetIdType::None
                    }
                }
            };
            if let Some(player) = user.player.as_ref() {
                all_played.push(CardResolve::new(*user_id, remote_id, card, target_id, player.attributes));
            }
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
