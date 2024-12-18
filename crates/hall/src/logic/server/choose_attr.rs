use crate::network::broadcaster::Broadcaster;
use hall::data::game::{GamePhase, GameState};
use hall::data::player::PlayerStatePlayerView;
use hall::message::GameResourcesMessage;
use shared_net::op;

pub(crate) fn handle_choose_attr(game: &mut GameState, bx: &mut Broadcaster) {
    let mut rng = rand::rng();
    let (erg_roll, users, remotes, mission) = game.split_borrow_for_resolve();
    for (id, user) in users.iter_mut() {
        if let Some(player) = user.player.as_ref() {
            if let Some(kind) = user.state.resolve_kind {
                if let Some(remote_id) = mission.remote_from_node(user.mission_state.node) {
                    if let Some(remote) = remotes.get(&remote_id) {
                        let (remote_attr, remote_kind) = remote.choose_attr(&mut rng);
                        let (local_erg, remote_erg) = GameState::resolve_matchups(erg_roll, &player.attributes.get(kind), &remote_attr);
                        user.state.add_erg(kind, local_erg);

                        let player_state_view = PlayerStatePlayerView::from(&user.state);
                        let message = GameResourcesMessage {
                            player_state_view,
                            remote_attr,
                            local_erg,
                            remote_erg,
                            remote_kind: remote_kind.into(),
                        };

                        bx.send_to_user(id, &message);
                    }
                }
            }
        }
    }
    game.set_phase(GamePhase::CardPlay, op::Command::GamePlayCard);
}
