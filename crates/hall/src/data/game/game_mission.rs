use crate::data::game::game_mission_node::GameMissionNode;
use crate::data::game::game_mission_objective::GameMissionObjective;
use crate::data::game::{GameMissionNodePlayerView, GameMissionObjectivePlayerView, RemoteIdType};
use crate::data::hall::HallMission;
use crate::data::player::PlayerMissionState;
use shared_data::mission::{MissionIdType, MissionNodeIdType};
use shared_net::sizedbuffers::Bufferable;
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

impl Bufferable for GameMissionPlayerView {
    fn push_into(&self, buf: &mut VSizedBuffer) {
        self.node.push_into(buf);
        self.objective.push_into(buf);
    }

    fn pull_from(buf: &mut VSizedBuffer) -> Self {
        let node = GameMissionNodePlayerView::pull_from(buf);
        let objective = <Vec<GameMissionObjectivePlayerView>>::pull_from(buf);
        Self {
            node,
            objective,
        }
    }

    fn size_in_buffer(&self) -> usize {
        self.node.size_in_buffer() + self.objective.size_in_buffer()
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
    use shared_net::sizedbuffers::Bufferable;
    use shared_net::VSizedBuffer;

    #[test]
    fn test_game_mission_player_view() {
        let orig_view = GameMissionPlayerView::test_default();

        let mut buf = VSizedBuffer::new(orig_view.size_in_buffer());
        buf.push(&orig_view);
        let new_view = buf.pull::<GameMissionPlayerView>();

        assert_eq!(orig_view, new_view);
    }
}
