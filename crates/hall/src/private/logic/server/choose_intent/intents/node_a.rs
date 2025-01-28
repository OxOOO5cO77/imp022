use hall::core::{AccessPointIntent, AuthLevel, TickType, Token, TokenKind, DEFAULT_TOKEN_EXPIRY};

use crate::private::game::{GameMission, GameUser};
use crate::private::logic::server::choose_intent::intents::IntentResult;

pub(crate) fn process_intent(intent: AccessPointIntent, mission: &mut GameMission, user: &mut GameUser, tick: TickType) -> Option<IntentResult> {
    match intent {
        AccessPointIntent::None => None,
        AccessPointIntent::Authenticate => process_authenticate(mission, user, tick),
    }
}

fn process_authenticate(_mission: &mut GameMission, user: &mut GameUser, tick: TickType) -> Option<IntentResult> {
    let mut messages = user.mission_state.upgrade_cred_to_auth();
    if !user.mission_state.any_auth() {
        let token = Token::new(TokenKind::Authorization(AuthLevel::Guest), tick + DEFAULT_TOKEN_EXPIRY);
        messages.push(user.mission_state.add_token(token));
    }
    Some(IntentResult::TokenChange(messages))
}
