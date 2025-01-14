use hall::core::{MissionIdType, MissionNodeIdType, MissionNodeState};
use hall::hall::HallMission;
use hall::view::{GameMissionObjectivePlayerView, GameMissionPlayerView};

use crate::private::game::{GameMissionNode, GameMissionObjective, GameUserMissionState};

#[derive(Default)]
pub(crate) struct GameMission {
    pub(crate) id: MissionIdType,
    pub(crate) node: Vec<GameMissionNode>,
    pub(crate) objective: Vec<GameMissionObjective>,
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
    pub(crate) fn get_node(&self, node: MissionNodeIdType) -> Option<&GameMissionNode> {
        self.node.iter().find(|n| n.id == node)
    }
    pub(crate) fn get_node_mut(&mut self, node: MissionNodeIdType) -> Option<&mut GameMissionNode> {
        self.node.iter_mut().find(|n| n.id == node)
    }
}

impl GameMission {
    pub(crate) fn to_player_state(&self, initial_node: MissionNodeIdType) -> GameUserMissionState {
        let mut state = GameUserMissionState {
            current_node: initial_node,
            known_nodes: self.node.iter().filter(|n| n.initial_state == MissionNodeState::Known).map(|n| n.id).collect(),
        };
        state.known_nodes.insert(initial_node);
        state
    }

    pub(crate) fn to_player_view(&self, mission_state: &GameUserMissionState) -> GameMissionPlayerView {
        let id = self.id;
        let current_node = mission_state.current();
        let node_map = mission_state.known_nodes.iter().filter_map(|id| self.get_node(*id)).map(|node| node.to_player_view()).collect();
        let objective = self.objective.iter().map(GameMissionObjectivePlayerView::from).collect();

        GameMissionPlayerView {
            id,
            current_node,
            node_map,
            objective,
        }
    }
}
