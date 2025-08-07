use hall_lib::core::Phase;
use hall_lib::message::GameResourcesMessage;

use crate::game::GameState;
use crate::network::broadcaster::Broadcaster;

pub(crate) fn handle_choose_attr(game: &mut GameState, bx: &mut Broadcaster) {
    let mut rng = rand::rng();
    let (erg_roll, users, remotes, mission) = game.split_borrow_for_resolve();
    for (id, user) in users.iter_mut() {
        if let Some(player) = user.player.as_ref()
            && let Some(kind) = user.state.resolve_kind
            && let Some(remote_id) = mission.get_node(user.mission_state.current()).map(|n| n.remote)
            && let Some(remote) = remotes.get(&remote_id)
        {
            let (remote_attr, remote_kind) = remote.choose_attr(&mut rng);
            let (local_erg, remote_erg) = GameState::resolve_matchups(erg_roll, &player.attributes.get_values(kind), &remote_attr);
            user.state.add_erg(kind, local_erg);

            let player_state_view = user.state.to_player_view();
            let message = GameResourcesMessage {
                player_state_view,
                remote_attr,
                local_erg,
                remote_erg,
                remote_kind,
            };

            bx.send_to_user(id, &message);
        }
    }
    game.set_phase(Phase::CardPlay);
}
