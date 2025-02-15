use std::collections::HashMap;

use hall_lib::message::{GameUpdateTokensMessage, UpdateTokenMessage};
use shared_net::UserIdType;

use crate::game::GameState;
use crate::network::broadcaster::Broadcaster;

pub(crate) fn some_users_update_tokens(game: &mut GameState, bx: &mut Broadcaster, mut users: HashMap<UserIdType, Vec<UpdateTokenMessage>>) {
    for (id, messages) in users.drain() {
        if game.users.contains_key(&id) {
            let message = GameUpdateTokensMessage::new(messages);
            bx.send_to_user(&id, &message);
        }
    }
}
