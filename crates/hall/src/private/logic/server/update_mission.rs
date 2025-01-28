use std::collections::HashSet;

use hall::message::GameUpdateMissionMessage;
use shared_net::UserIdType;

use crate::private::game::GameState;
use crate::private::network::broadcaster::Broadcaster;

pub(crate) fn some_users_update_mission(game: &mut GameState, bx: &mut Broadcaster, users: HashSet<UserIdType>) {
    for id in &users {
        if game.users.contains_key(id) {
            let message = GameUpdateMissionMessage::new(true);
            bx.send_to_user(id, &message);
        }
    }
}
