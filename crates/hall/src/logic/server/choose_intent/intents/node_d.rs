use hall_lib::core::DatabaseIntent;

use crate::game::{GameMission, GameUser};
use crate::logic::server::choose_intent::intents::IntentResult;

pub(crate) fn process_intent(_intent: DatabaseIntent, _mission: &mut GameMission, _user: &mut GameUser) -> Option<Vec<IntentResult>> {
    None
}
