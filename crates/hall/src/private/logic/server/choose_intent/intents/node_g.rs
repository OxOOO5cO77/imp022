use hall::core::GatewayIntent;

use crate::private::game::{GameMission, GameUser};
use crate::private::logic::server::choose_intent::intents::IntentResult;

pub(crate) fn process_intent(_intent: GatewayIntent, _mission: &mut GameMission, _user: &mut GameUser) -> Option<IntentResult> {
    None
}
