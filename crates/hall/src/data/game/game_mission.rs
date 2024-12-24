use crate::data::core::{MissionIdType, MissionNodeIdType};
use crate::data::game::game_mission_node::GameMissionNode;
use crate::data::game::game_mission_objective::GameMissionObjective;
use crate::data::game::{GameMissionNodePlayerView, GameMissionObjectivePlayerView, RemoteIdType};
use crate::data::hall::HallMission;
use crate::data::player::PlayerMissionState;
use shared_net::Bufferable;
use shared_net::VSizedBuffer;

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
    pub fn remote_from_node(&self, node: MissionNodeIdType) -> Option<RemoteIdType> {
        self.node.iter().find(|n| n.id == node).map(|n| n.remote)
    }
}

#[derive(Bufferable)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameMissionPlayerView {
    pub node: GameMissionNodePlayerView,
    objective: Vec<GameMissionObjectivePlayerView>,
}

impl GameMissionPlayerView {
    pub fn new(mission: &GameMission, mission_state: &PlayerMissionState) -> Self {
        Self {
            node: mission.node.iter().find(|n| n.id == mission_state.node).map(GameMissionNodePlayerView::from).unwrap(),
            objective: mission.objective.iter().map(GameMissionObjectivePlayerView::from).collect(),
        }
    }
}

#[cfg(test)]
impl GameMissionPlayerView {
    pub fn test_default() -> Self {
        Self {
            node: GameMissionNodePlayerView::test_default(),
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
