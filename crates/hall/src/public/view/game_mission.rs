use crate::core::{MissionIdType, MissionNodeIdType, MissionNodeKind};
use crate::view::{GameMissionNodePlayerView, GameMissionObjectivePlayerView};
use shared_net::Bufferable;
use shared_net::VSizedBuffer;

#[derive(Bufferable, Default, Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameMissionPlayerView {
    pub id: MissionIdType,
    pub current_node: MissionNodeIdType,
    pub node_map: Vec<GameMissionNodePlayerView>,
    pub objective: Vec<GameMissionObjectivePlayerView>,
}

static UNKNOWN_NODE: GameMissionNodePlayerView = GameMissionNodePlayerView {
    id: 0,
    kind: MissionNodeKind::Unknown,
    links: vec![],
    content: vec![],
    remote: 0,
};

impl GameMissionPlayerView {
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
            id: 1234567890,
            current_node: 123,
            node_map: vec![GameMissionNodePlayerView::test_default(), GameMissionNodePlayerView::test_default(), GameMissionNodePlayerView::test_default()],
            objective: vec![GameMissionObjectivePlayerView::test_default(), GameMissionObjectivePlayerView::test_default(), GameMissionObjectivePlayerView::test_default()],
        }
    }
}

#[cfg(test)]
mod test {
    use crate::view::GameMissionPlayerView;
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
