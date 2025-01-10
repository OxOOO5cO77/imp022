use shared_net::Bufferable;
use shared_net::VSizedBuffer;

use crate::data::core::{MissionIdType, MissionNodeIdType, MissionNodeKind};
use crate::data::game::game_mission_node::GameMissionNode;
use crate::data::game::game_mission_objective::GameMissionObjective;
use crate::data::game::{GameMissionNodePlayerView, GameMissionObjectivePlayerView};
use crate::data::hall::HallMission;
use crate::data::player::PlayerMissionState;

#[derive(Default)]
pub struct GameMission {
    pub id: MissionIdType,
    pub node: Vec<GameMissionNode>,
    pub objective: Vec<GameMissionObjective>,
}

impl From<HallMission> for GameMission {
    fn from(value: HallMission) -> Self {
        Self {
            id: value.id,
            node: value.node.iter().map(|node| GameMissionNode::new(node, 0)).collect(),
            objective: value.objective.iter().map(GameMissionObjective::from).collect(),
        }
    }
}

impl GameMission {
    pub fn get_node(&self, node: MissionNodeIdType) -> Option<&GameMissionNode> {
        self.node.iter().find(|n| n.id == node)
    }
}

#[derive(Bufferable, Default, Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameMissionPlayerView {
    current_node: MissionNodeIdType,
    node_map: Vec<GameMissionNodePlayerView>,
    objective: Vec<GameMissionObjectivePlayerView>,
}

static UNKNOWN_NODE: GameMissionNodePlayerView = GameMissionNodePlayerView {
    id: 0,
    kind: MissionNodeKind::Unknown,
    links: vec![],
    content: vec![],
    remote: 0,
};

impl GameMissionPlayerView {
    pub fn new(mission: &GameMission, mission_state: &PlayerMissionState) -> Self {
        let current_node = mission_state.current();
        let node_map = mission_state.known().iter().filter_map(|id| mission.get_node(*id)).map(GameMissionNodePlayerView::new).collect();
        let objective = mission.objective.iter().map(GameMissionObjectivePlayerView::from).collect();

        Self {
            current_node,
            node_map,
            objective,
        }
    }

    pub fn current(&self) -> &GameMissionNodePlayerView {
        self.get_node(self.current_node).unwrap_or(&UNKNOWN_NODE)
    }
    pub fn get_node(&self, node: MissionNodeIdType) -> Option<&GameMissionNodePlayerView> {
        self.node_map.iter().find(|n| n.id == node)
    }
}

#[cfg(test)]
impl GameMissionPlayerView {
    pub fn test_default() -> Self {
        Self {
            current_node: 4,
            node_map: vec![GameMissionNodePlayerView::test_default(), GameMissionNodePlayerView::test_default(), GameMissionNodePlayerView::test_default()],
            objective: vec![GameMissionObjectivePlayerView::test_default(), GameMissionObjectivePlayerView::test_default(), GameMissionObjectivePlayerView::test_default()],
        }
    }
}

#[cfg(test)]
mod test {
    use crate::data::game::GameMissionPlayerView;
    use shared_net::{Bufferable, VSizedBuffer};

    #[test]
    fn test_game_mission_player_view() {
        let orig_view = GameMissionPlayerView::test_default();

        let mut buf = VSizedBuffer::new(orig_view.size_in_buffer());
        buf.push(&orig_view);
        let new_view = buf.pull::<GameMissionPlayerView>();

        assert_eq!(orig_view, new_view);
    }
}
