use crate::data::game::RemoteIdType;
use crate::data::hall::HallMissionNode;
use shared_data::mission::{MissionNodeContent, MissionNodeIdType, MissionNodeKind, MissionNodeLink, MissionNodeLinkDir, MissionNodeLinkState, MissionNodeState};
use shared_net::sizedbuffers::Bufferable;
use shared_net::VSizedBuffer;

pub struct GameMissionNode {
    pub id: MissionNodeIdType,
    pub kind: MissionNodeKind,
    pub state: MissionNodeState,
    pub links: Vec<MissionNodeLink>,
    pub content: Vec<MissionNodeContent>,
    pub remote: RemoteIdType,
}

impl GameMissionNode {
    pub(crate) fn new(node: &HallMissionNode, remote: RemoteIdType) -> Self {
        Self {
            id: node.id,
            kind: node.kind,
            state: node.state,
            links: node.links.clone(),
            content: node.content.clone(),
            remote,
        }
    }
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameMissionNodePlayerView {
    pub id: MissionNodeIdType,
    pub kind: MissionNodeKind,
    state: MissionNodeState,
    links: Vec<MissionNodeLink>,
    pub remote: RemoteIdType,
}

impl From<&GameMissionNode> for GameMissionNodePlayerView {
    fn from(value: &GameMissionNode) -> Self {
        Self {
            id: value.id,
            kind: value.kind,
            state: value.state,
            links: value.links.clone(),
            remote: value.remote,
        }
    }
}

type PackedMissionType = u32;

impl GameMissionNodePlayerView {
    fn pack_kind(kind: MissionNodeKind) -> PackedMissionType {
        match kind {
            MissionNodeKind::AccessPoint => 0,
            MissionNodeKind::Backend => 1,
            MissionNodeKind::Control => 2,
            MissionNodeKind::Database => 3,
            MissionNodeKind::Engine => 4,
            MissionNodeKind::Frontend => 5,
            MissionNodeKind::Gateway => 6,
            MissionNodeKind::Hardware => 7,
        }
    }
    fn unpack_kind(packed: PackedMissionType) -> MissionNodeKind {
        match packed {
            0 => MissionNodeKind::AccessPoint,
            1 => MissionNodeKind::Backend,
            2 => MissionNodeKind::Control,
            3 => MissionNodeKind::Database,
            4 => MissionNodeKind::Engine,
            5 => MissionNodeKind::Frontend,
            6 => MissionNodeKind::Gateway,
            7 => MissionNodeKind::Hardware,
            _ => MissionNodeKind::AccessPoint,
        }
    }

    fn pack_state(state: MissionNodeState) -> PackedMissionType {
        match state {
            MissionNodeState::Unknown => 0,
            MissionNodeState::Known => 1,
        }
    }
    fn unpack_state(packed: PackedMissionType) -> MissionNodeState {
        match packed {
            0 => MissionNodeState::Unknown,
            1 => MissionNodeState::Known,
            _ => MissionNodeState::Unknown,
        }
    }

    fn pack_link_dir(dir: MissionNodeLinkDir) -> PackedMissionType {
        match dir {
            MissionNodeLinkDir::North => 0,
            MissionNodeLinkDir::East => 1,
            MissionNodeLinkDir::South => 2,
            MissionNodeLinkDir::West => 3,
        }
    }
    fn unpack_link_dir(packed: PackedMissionType) -> MissionNodeLinkDir {
        match packed {
            0 => MissionNodeLinkDir::North,
            1 => MissionNodeLinkDir::East,
            2 => MissionNodeLinkDir::South,
            3 => MissionNodeLinkDir::West,
            _ => MissionNodeLinkDir::North,
        }
    }
    fn pack_link_state(state: MissionNodeLinkState) -> PackedMissionType {
        match state {
            MissionNodeLinkState::Closed => 0,
            MissionNodeLinkState::Open => 1,
        }
    }
    fn unpack_link_state(packed: PackedMissionType) -> MissionNodeLinkState {
        match packed {
            0 => MissionNodeLinkState::Closed,
            1 => MissionNodeLinkState::Open,
            _ => MissionNodeLinkState::Closed,
        }
    }

    fn pack_link(link: &MissionNodeLink) -> PackedMissionType {
        let mut packed = 0;
        packed |= Self::pack_link_dir(link.direction) << 14;
        packed |= Self::pack_link_state(link.state) << 13;
        packed |= link.target as PackedMissionType;
        packed
    }
    fn unpack_link(packed: PackedMissionType) -> MissionNodeLink {
        MissionNodeLink {
            direction: Self::unpack_link_dir((packed >> 14) & 0x3),
            target: (packed & 0xFF) as MissionNodeIdType,
            state: Self::unpack_link_state((packed >> 13) & 0x1),
        }
    }

    fn pack_links(links: &[MissionNodeLink]) -> PackedMissionType {
        let mut packed = 0;
        for (i, link) in links.iter().enumerate() {
            packed |= Self::pack_link(link) << (i * 4);
        }
        packed
    }
    fn unpack_links(packed: PackedMissionType) -> Vec<MissionNodeLink> {
        vec![
            //
            Self::unpack_link(packed & 0xF),
            Self::unpack_link((packed >> 4) & 0xF),
            Self::unpack_link((packed >> 8) & 0xF),
            Self::unpack_link((packed >> 12) & 0xF),
        ]
    }

    fn pack_mission(&self) -> PackedMissionType {
        let packed_id = (self.id as PackedMissionType) << 24;
        if self.state == MissionNodeState::Unknown {
            return packed_id;
        }
        let packed_kind = Self::pack_kind(self.kind) << 20;
        let packed_state = Self::pack_state(self.state) << 16;
        let packed_links = Self::pack_links(&self.links);
        packed_id | packed_kind | packed_state | packed_links
    }

    fn unpack_mission(packed: PackedMissionType) -> GameMissionNodePlayerView {
        let id = ((packed >> 24) & 0xFF) as MissionNodeIdType;
        let kind = Self::unpack_kind((packed >> 20) & 0xF);
        let state = Self::unpack_state((packed >> 16) & 0xF);
        let links = Self::unpack_links(packed & 0xFFFF);
        GameMissionNodePlayerView {
            id,
            kind,
            state,
            links,
            remote: 0,
        }
    }
}

impl Bufferable for GameMissionNodePlayerView {
    fn push_into(&self, buf: &mut VSizedBuffer) {
        let packed = self.pack_mission();
        packed.push_into(buf);
        self.remote.push_into(buf);
    }

    fn pull_from(buf: &mut VSizedBuffer) -> Self {
        let packed = PackedMissionType::pull_from(buf);
        let mut unpacked = GameMissionNodePlayerView::unpack_mission(packed);
        unpacked.remote = RemoteIdType::pull_from(buf);
        unpacked
    }

    fn size_in_buffer(&self) -> usize {
        size_of::<PackedMissionType>() + self.remote.size_in_buffer()
    }
}

#[cfg(test)]
impl GameMissionNodePlayerView {
    pub fn test_default() -> Self {
        Self {
            id: 123,
            kind: MissionNodeKind::Frontend,
            state: MissionNodeState::Known,
            links: vec![
                MissionNodeLink {
                    direction: MissionNodeLinkDir::North,
                    target: 4,
                    state: MissionNodeLinkState::Closed,
                },
                MissionNodeLink {
                    direction: MissionNodeLinkDir::East,
                    target: 3,
                    state: MissionNodeLinkState::Open,
                },
                MissionNodeLink {
                    direction: MissionNodeLinkDir::South,
                    target: 2,
                    state: MissionNodeLinkState::Open,
                },
                MissionNodeLink {
                    direction: MissionNodeLinkDir::West,
                    target: 1,
                    state: MissionNodeLinkState::Closed,
                },
            ],
            remote: 12345678901234567890,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::data::game::GameMissionNodePlayerView;
    use shared_net::sizedbuffers::Bufferable;
    use shared_net::VSizedBuffer;

    #[test]
    fn test_game_mission_node_player_view() {
        let orig_view = GameMissionNodePlayerView::test_default();

        let mut buf = VSizedBuffer::new(orig_view.size_in_buffer());
        buf.push(&orig_view);
        let new_view = buf.pull::<GameMissionNodePlayerView>();

        assert_eq!(orig_view, new_view);
    }
}
