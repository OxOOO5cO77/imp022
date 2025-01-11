use hall::core::MissionNodeLinkDir;

use crate::private::game::{GameMission, GameUser};

pub(crate) fn process_intent(mission: &mut GameMission, user: &mut GameUser, dir: MissionNodeLinkDir) -> bool {
    mission //
        .get_node(user.mission_state.current())
        .and_then(|mission| mission.links.iter().find(|n| n.direction == dir))
        .map(|link| user.mission_state.set_current(link.target))
        .unwrap_or(false)
}
