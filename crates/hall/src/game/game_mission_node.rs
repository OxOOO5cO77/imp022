use crate::game::game_state::ActorMapType;
use hall_lib::core::{ActorIdType, AuthLevel, MissionNodeContent, MissionNodeIdType, MissionNodeKind, MissionNodeLink, MissionNodeState, RemoteIdType};
use hall_lib::view::{GameMissionNodePlayerView, MissionNodeLinkView};

#[derive(Default)]
pub struct GameMissionNode {
    pub id: MissionNodeIdType,
    pub kind: MissionNodeKind,
    pub initial_state: MissionNodeState,
    pub links: Vec<MissionNodeLink>,
    pub content: Vec<MissionNodeContent>,
    pub remote: RemoteIdType,
    pub actors: Vec<ActorIdType>,
}

impl GameMissionNode {
    fn link_to_player_view(link: &MissionNodeLink, auth_level: AuthLevel) -> MissionNodeLinkView {
        MissionNodeLinkView {
            direction: link.direction,
            target: link.target,
            min_level: link.min_level,
            locked: auth_level < link.min_level,
        }
    }

    pub(crate) fn to_player_view(&self, auth_level: AuthLevel, all_actors: &ActorMapType) -> GameMissionNodePlayerView {
        GameMissionNodePlayerView {
            id: self.id,
            kind: self.kind,
            links: self.links.iter().map(|node| Self::link_to_player_view(node, auth_level)).collect(),
            content: self.content.clone(),
            remote: self.remote,
            actors: self.actors.iter().filter_map(|id| all_actors.get(id).map(|a| a.to_player_view(*id))).collect(),
        }
    }
}
