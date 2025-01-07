use crate::network::broadcaster::Broadcaster;
use hall::data::game::GameState;
use hall::message::GameUpdateStateResponse;

pub(crate) fn all_users_update_state(game: &mut GameState, bx: &mut Broadcaster) {
    for (id, user) in game.users.iter() {
        if let Some(remote_id) = game.mission.get_node(user.mission_state.current()).map(|r| r.remote) {
            if let Some(remote) = game.remotes.get(&remote_id) {
                let message = GameUpdateStateResponse::new(user, &remote.machine, &game.mission);
                bx.send_to_user(id, &message);
            }
        }
    }
}
