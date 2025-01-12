use hall::core::{MissionIdType, MissionNodeIdType, MissionNodeState};
use hall::hall::HallMission;
use hall::player::PlayerMissionState;
use hall::view::{GameMissionObjectivePlayerView, GameMissionPlayerView};

use crate::private::game::{GameMissionNode, GameMissionObjective};

#[derive(Default)]
pub(crate) struct GameMission {
    pub(crate) _id: MissionIdType,
    pub(crate) node: Vec<GameMissionNode>,
    pub(crate) objective: Vec<GameMissionObjective>,
}

impl From<HallMission> for GameMission {
    fn from(value: HallMission) -> Self {
        Self {
            _id: value.id,
            node: value.node.iter().map(|node| GameMissionNode::new(node, 0)).collect(),
            objective: value.objective.iter().map(GameMissionObjective::from).collect(),
        }
    }
}

impl GameMission {
    pub(crate) fn get_node(&self, node: MissionNodeIdType) -> Option<&GameMissionNode> {
        self.node.iter().find(|n| n.id == node)
    }
    pub(crate) fn get_node_mut(&mut self, node: MissionNodeIdType) -> Option<&mut GameMissionNode> {
        self.node.iter_mut().find(|n| n.id == node)
    }
}

impl GameMission {
    pub(crate) fn to_player_state(&self, initial_node: MissionNodeIdType) -> PlayerMissionState {
        let mut state = PlayerMissionState {
            current_node: initial_node,
            nodes: self.node.iter().filter(|n| n.initial_state == MissionNodeState::Known).map(|n| (n.id, MissionNodeState::Known)).collect(),
        };
        state.nodes.insert(initial_node, MissionNodeState::Known);
        state
    }

    pub(crate) fn to_player_view(&self, mission_state: &PlayerMissionState) -> GameMissionPlayerView {
        let current_node = mission_state.current();
        let node_map = mission_state.known().iter().filter_map(|id| self.get_node(*id)).map(|node| node.to_player_view()).collect();
        let objective = self.objective.iter().map(GameMissionObjectivePlayerView::from).collect();

        GameMissionPlayerView {
            current_node,
            node_map,
            objective,
        }
    }
}
