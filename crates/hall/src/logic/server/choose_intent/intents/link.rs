use hall_lib::core::MissionNodeLinkDir;

use crate::game::{GameMission, GameUser};
use crate::logic::server::choose_intent::intents::IntentResult;

pub(crate) fn process_intent(dir: MissionNodeLinkDir, user: &mut GameUser, mission: &mut GameMission) -> Option<Vec<IntentResult>> {
    let node = mission.get_node_mut(user.mission_state.current())?;
    let link = node.links.iter_mut().find(|n| n.direction == dir)?;
    if user.mission_state.max_auth_level() >= link.min_level {
        user.mission_state.set_current(link.target);
        Some(vec![IntentResult::Node])
    } else {
        None
    }
}
