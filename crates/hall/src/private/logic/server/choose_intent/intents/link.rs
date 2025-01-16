use hall::core::{MissionNodeLinkDir, MissionNodeLinkState};
use hall::view::MAX_LINK_DAMAGE;

use crate::private::game::{GameMission, GameUser, RemoteMapType};
use crate::private::logic::server::choose_intent::intents::IntentResult;

pub(crate) fn process_intent(dir: MissionNodeLinkDir, user: &mut GameUser, mission: &mut GameMission, remotes: &mut RemoteMapType) -> Option<IntentResult> {
    let node = mission.get_node_mut(user.mission_state.current())?;
    let link = node.links.iter_mut().find(|n| n.direction == dir)?;
    match link.state {
        MissionNodeLinkState::Open => {
            user.mission_state.set_current(link.target);
            Some(IntentResult::NodeChange)
        }
        MissionNodeLinkState::Closed => {
            let remote = remotes.get_mut(&node.remote)?;
            let amount = user.player.as_ref()?.attributes.breach.amplitude.saturating_sub(remote.attributes.breach.control).max(1);
            link.damage = link.damage.saturating_add(amount).min(MAX_LINK_DAMAGE);
            if link.damage == MAX_LINK_DAMAGE {
                link.state = MissionNodeLinkState::Open;
            }
            None
        }
    }
}
