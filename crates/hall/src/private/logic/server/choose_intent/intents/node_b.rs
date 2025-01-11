use hall::core::BackendIntent;

use crate::private::game::{GameMission, GameUser};

pub(crate) fn process_intent(_mission: &mut GameMission, _user: &mut GameUser, _intent: BackendIntent) -> bool {
    false
}
