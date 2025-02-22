use hall_lib::core::{AccessPointIntent, AuthLevel, DEFAULT_TOKEN_EXPIRY, MissionNodeKind, Rarity, TickType, Token, TokenKind};
use hall_lib::player::PlayerCard;

use crate::game::{GameMission, GameUser};
use crate::logic::server::choose_intent::intents::IntentResult;
use crate::manager::data_manager::DataManager;

pub(crate) fn process_intent(intent: AccessPointIntent, mission: &mut GameMission, user: &mut GameUser, tick: TickType, dm: &DataManager) -> Option<Vec<IntentResult>> {
    match intent {
        AccessPointIntent::None => None,
        AccessPointIntent::Authenticate => process_authenticate(mission, user, tick, dm),
        AccessPointIntent::TransferNext => process_transfer(mission, user, 1),
        AccessPointIntent::TransferPrev => process_transfer(mission, user, -1),
    }
}

const CARD_REAUTHORIZE: PlayerCard = PlayerCard::new(0, Rarity::Common, 1);

fn process_authenticate(_mission: &mut GameMission, user: &mut GameUser, tick: TickType, dm: &DataManager) -> Option<Vec<IntentResult>> {
    let mut messages = user.mission_state.upgrade_cred_to_auth();
    if !user.mission_state.any_auth() {
        let token = Token::new(TokenKind::Authorization(AuthLevel::Guest), tick + DEFAULT_TOKEN_EXPIRY);
        messages.push(user.mission_state.add_token(token));
    }

    if let Some(card) = dm.lookup_player_card(&CARD_REAUTHORIZE) {
        user.state.add_to_deck(card.clone(), 10);
    }
    Some(vec![IntentResult::Token(messages), IntentResult::Deck])
}

fn process_transfer(mission: &mut GameMission, user: &mut GameUser, offset: isize) -> Option<Vec<IntentResult>> {
    let aps = mission.node.iter().filter(|n| n.kind == MissionNodeKind::AccessPoint).map(|n| n.id).collect::<Vec<_>>();
    if let Some(cur_index) = aps.iter().position(|n| *n == user.mission_state.current()) {
        let new_index = (cur_index + aps.len()).saturating_add_signed(offset) % aps.len();
        user.mission_state.set_current(aps[new_index]);
        return Some(vec![IntentResult::Node]);
    }
    None
}
