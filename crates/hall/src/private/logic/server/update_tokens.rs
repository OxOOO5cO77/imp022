use hall::core::Token;
use hall::message::GameUpdateTokensMessage;
use shared_net::UserIdType;
use std::collections::HashMap;

use crate::private::game::GameState;
use crate::private::network::broadcaster::Broadcaster;

pub(crate) fn some_users_update_tokens(game: &mut GameState, bx: &mut Broadcaster, users: HashMap<&UserIdType, Token>) {
    for (id, token) in &users {
        if game.users.contains_key(id) {
            let message = GameUpdateTokensMessage::new((*token).clone());
            bx.send_to_user(id, &message);
        }
    }
}
