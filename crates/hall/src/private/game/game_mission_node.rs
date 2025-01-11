use hall::core::{MissionNodeContent, MissionNodeIdType, MissionNodeKind, MissionNodeLink, MissionNodeState, RemoteIdType};
use hall::hall::HallMissionNode;
use hall::view::GameMissionNodePlayerView;

pub struct GameMissionNode {
    pub id: MissionNodeIdType,
    pub kind: MissionNodeKind,
    pub initial_state: MissionNodeState,
    pub links: Vec<MissionNodeLink>,
    pub content: Vec<MissionNodeContent>,
    pub remote: RemoteIdType,
}

impl GameMissionNode {
    pub(crate) fn new(node: &HallMissionNode, remote: RemoteIdType) -> Self {
        Self {
            id: node.id,
            kind: node.kind,
            initial_state: node.state,
            links: node.links.clone(),
            content: node.content.clone(),
            remote,
        }
    }
}

impl GameMissionNode {
    pub(crate) fn to_player_view(&self) -> GameMissionNodePlayerView {
        GameMissionNodePlayerView {
            id: self.id,
            kind: self.kind,
            links: self.links.clone(),
            content: self.content.clone(),
            remote: self.remote,
        }
    }
}
