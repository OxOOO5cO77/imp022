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
    let cred = user.mission_state.get_token(TokenKind::Credentials(AuthLevel::Guest));
    let kind = match cred {
        Some(existing) => match existing.kind {
            TokenKind::Credentials(level) => TokenKind::Authorization(level),
            _ => TokenKind::Authorization(AuthLevel::Guest),
        },
        None => TokenKind::Authorization(AuthLevel::Guest),
    };
    let token = Token::new(kind, tick + DEFAULT_TOKEN_EXPIRY);
    user.mission_state.add_token(token.clone());
    Some(IntentResult::TokenChange(token))
}
