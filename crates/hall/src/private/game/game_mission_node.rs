use crate::private::game::game_state::ActorMapType;
use hall::core::{ActorIdType, MissionNodeContent, MissionNodeIdType, MissionNodeKind, MissionNodeLink, MissionNodeState, RemoteIdType};
use hall::view::GameMissionNodePlayerView;

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
    pub(crate) fn to_player_view(&self, all_actors: &ActorMapType) -> GameMissionNodePlayerView {
        GameMissionNodePlayerView {
            id: self.id,
            kind: self.kind,
            links: self.links.clone(),
            content: self.content.clone(),
            remote: self.remote,
            actors: self.actors.iter().filter_map(|id| all_actors.get(id).map(|a| a.to_player_view(*id))).collect(),
        }
    }
}
