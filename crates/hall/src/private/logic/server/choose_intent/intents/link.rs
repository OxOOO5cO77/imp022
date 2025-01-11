use hall::core::{MissionNodeLinkDir, MissionNodeLinkState};

use crate::private::game::{GameMission, GameUser};

pub(crate) fn process_intent(mission: &mut GameMission, user: &mut GameUser, dir: MissionNodeLinkDir) -> bool {
    mission // note: we don't care if the target node is new, just that we traversed successfully
        .get_node(user.mission_state.current())
        .and_then(|mission| mission.links.iter().find(|n| n.direction == dir))
        .filter(|link| link.state == MissionNodeLinkState::Open)
        .map(|link| user.mission_state.set_current(link.target))
        .is_some()
}
