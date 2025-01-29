use hall::core::{GeneralType, MissionIdType, MissionNodeIdType, MissionNodeState, SpecificType};
use hall::hall::HallMission;
use hall::view::{GameMissionObjectivePlayerView, GameMissionPlayerView};

use crate::private::game::game_state::ActorMapType;
use crate::private::game::{GameMissionNode, GameMissionObjective, GameUserMissionState};

#[derive(Default)]
pub(crate) struct GameMission {
    pub(crate) id: MissionIdType,
    pub(crate) institution: (GeneralType, SpecificType),
    pub(crate) node: Vec<GameMissionNode>,
    pub(crate) objective: Vec<GameMissionObjective>,
}

impl GameMission {
    pub(crate) fn new(value: &HallMission, institution: (GeneralType, SpecificType)) -> Self {
        Self {
            id: value.id,
            institution,
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
            tokens: Vec::new(),
        };
        state.known_nodes.insert(initial_node);
        state
    }

    pub(crate) fn to_player_view(&self, mission_state: &GameUserMissionState, actors: &ActorMapType) -> GameMissionPlayerView {
        let id = self.id;
        let institution = self.institution;
        let current_node = mission_state.current();
        let node_map = mission_state.known_nodes.iter().filter_map(|id| self.get_node(*id)).map(|node| node.to_player_view(actors)).collect();
        let tokens = mission_state.tokens.clone();
        let objective = self.objective.iter().map(GameMissionObjectivePlayerView::from).collect();

        GameMissionPlayerView {
            id,
            institution,
            current_node,
            node_map,
            tokens,
            objective,
        }
    }
}
