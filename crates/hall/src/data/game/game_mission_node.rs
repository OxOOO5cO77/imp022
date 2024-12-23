use crate::data::core::{MissionNodeContent, MissionNodeIdType, MissionNodeKind, MissionNodeLink, MissionNodeLinkDir, MissionNodeLinkState, MissionNodeState};
use crate::data::game::RemoteIdType;
use crate::data::hall::HallMissionNode;
use shared_net::{Bufferable, VSizedBuffer};

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

type PackedMissionType = u64;

const MAX_LINK_COUNT: PackedMissionType = 4;

const BITS_FOR_ID: PackedMissionType = 8;
const BITS_FOR_KIND: PackedMissionType = 5;
const BITS_FOR_STATE: PackedMissionType = 1;
const BITS_FOR_LINK_DIR: PackedMissionType = 2;
const BITS_FOR_LINK_STATE: PackedMissionType = 1;
const BITS_FOR_LINK_TARGET: PackedMissionType = 8;
const BITS_FOR_LINK: PackedMissionType = BITS_FOR_LINK_DIR + BITS_FOR_LINK_STATE + BITS_FOR_LINK_TARGET;
const BITS_FOR_LINKS: PackedMissionType = BITS_FOR_LINK * 4;

const SHIFT_FOR_ID: PackedMissionType = 0;
const SHIFT_FOR_KIND: PackedMissionType = SHIFT_FOR_ID + BITS_FOR_ID;
const SHIFT_FOR_STATE: PackedMissionType = SHIFT_FOR_KIND + BITS_FOR_KIND;
const SHIFT_FOR_LINKS: PackedMissionType = SHIFT_FOR_STATE + BITS_FOR_STATE;
const SHIFT_FOR_LINK_DIR: PackedMissionType = 0;
const SHIFT_FOR_LINK_STATE: PackedMissionType = SHIFT_FOR_LINK_DIR + BITS_FOR_LINK_DIR;
const SHIFT_FOR_LINK_TARGET: PackedMissionType = SHIFT_FOR_LINK_STATE + BITS_FOR_LINK_STATE;

const MASK_FOR_ID: PackedMissionType = (1 << BITS_FOR_ID) - 1;
const MASK_FOR_KIND: PackedMissionType = (1 << BITS_FOR_KIND) - 1;
const MASK_FOR_STATE: PackedMissionType = (1 << BITS_FOR_STATE) - 1;
const MASK_FOR_LINK_DIR: PackedMissionType = (1 << BITS_FOR_LINK_DIR) - 1;
const MASK_FOR_LINK_STATE: PackedMissionType = (1 << BITS_FOR_LINK_STATE) - 1;
const MASK_FOR_LINK_TARGET: PackedMissionType = (1 << BITS_FOR_LINK_TARGET) - 1;
const MASK_FOR_LINKS: PackedMissionType = (1 << BITS_FOR_LINKS) - 1;
const MASK_FOR_LINK: PackedMissionType = (1 << BITS_FOR_LINK) - 1;

impl GameMissionNodePlayerView {
    fn pack_kind(kind: MissionNodeKind) -> PackedMissionType {
        let kind: u8 = kind.into();
        kind as PackedMissionType
    }
    fn unpack_kind(packed: PackedMissionType) -> MissionNodeKind {
        (packed as u8).into()
    }

    fn pack_state(state: MissionNodeState) -> PackedMissionType {
        let state: u8 = state.into();
        state as PackedMissionType
    }
    fn unpack_state(packed: PackedMissionType) -> MissionNodeState {
        (packed as u8).into()
    }

    fn pack_link_dir(dir: MissionNodeLinkDir) -> PackedMissionType {
        let dir: u8 = dir.into();
        dir as PackedMissionType
    }
    fn unpack_link_dir(packed: PackedMissionType) -> MissionNodeLinkDir {
        (packed as u8).into()
    }

    fn pack_link_state(state: MissionNodeLinkState) -> PackedMissionType {
        let state: u8 = state.into();
        state as PackedMissionType
    }
    fn unpack_link_state(packed: PackedMissionType) -> MissionNodeLinkState {
        (packed as u8).into()
    }

    fn pack_link(link: &MissionNodeLink) -> PackedMissionType {
        let mut packed = 0;
        packed |= Self::pack_link_dir(link.direction) << SHIFT_FOR_LINK_DIR;
        packed |= Self::pack_link_state(link.state) << SHIFT_FOR_LINK_STATE;
        packed |= (link.target as PackedMissionType) << SHIFT_FOR_LINK_TARGET;
        packed
    }
    fn unpack_link(packed: PackedMissionType) -> MissionNodeLink {
        MissionNodeLink {
            direction: Self::unpack_link_dir((packed >> SHIFT_FOR_LINK_DIR) & MASK_FOR_LINK_DIR),
            target: ((packed >> SHIFT_FOR_LINK_TARGET) & MASK_FOR_LINK_TARGET) as MissionNodeIdType,
            state: Self::unpack_link_state((packed >> SHIFT_FOR_LINK_STATE) & MASK_FOR_LINK_STATE),
        }
    }

    fn pack_links(links: &[MissionNodeLink]) -> PackedMissionType {
        let mut packed = 0;
        for (i, link) in links.iter().enumerate() {
            packed |= Self::pack_link(link) << (i as PackedMissionType * BITS_FOR_LINK);
        }
        packed
    }
    fn unpack_links(packed: PackedMissionType) -> Vec<MissionNodeLink> {
        let mut links = Vec::new();
        for i in 0..MAX_LINK_COUNT {
            let link = Self::unpack_link((packed >> (BITS_FOR_LINK * i)) & MASK_FOR_LINK);
            if link.target != 0 {
                links.push(link);
            }
        }
        links
    }

    fn pack_mission(&self) -> PackedMissionType {
        let packed_id = (self.id as PackedMissionType) << SHIFT_FOR_ID;
        if self.state == MissionNodeState::Unknown {
            return packed_id;
        }
        let packed_kind = Self::pack_kind(self.kind) << SHIFT_FOR_KIND;
        let packed_state = Self::pack_state(self.state) << SHIFT_FOR_STATE;
        let packed_links = Self::pack_links(&self.links) << SHIFT_FOR_LINKS;
        packed_id | packed_kind | packed_state | packed_links
    }

    fn unpack_mission(packed: PackedMissionType) -> (MissionNodeIdType, MissionNodeKind, MissionNodeState, Vec<MissionNodeLink>) {
        let id = ((packed >> SHIFT_FOR_ID) & MASK_FOR_ID) as MissionNodeIdType;
        let kind = Self::unpack_kind((packed >> SHIFT_FOR_KIND) & MASK_FOR_KIND);
        let state = Self::unpack_state((packed >> SHIFT_FOR_STATE) & MASK_FOR_STATE);
        let links = Self::unpack_links((packed >> SHIFT_FOR_LINKS) & MASK_FOR_LINKS);
        (id, kind, state, links)
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
        let (id, kind, state, links) = Self::unpack_mission(packed);
        let remote = RemoteIdType::pull_from(buf);
        Self {
            id,
            kind,
            state,
            links,
            remote,
        }
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
                    target: 124,
                    state: MissionNodeLinkState::Closed,
                },
                MissionNodeLink {
                    direction: MissionNodeLinkDir::East,
                    target: 3,
                    state: MissionNodeLinkState::Open,
                },
                MissionNodeLink {
                    direction: MissionNodeLinkDir::South,
                    target: 234,
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
    use shared_net::{Bufferable, VSizedBuffer};

    #[test]
    fn test_game_mission_node_player_view() {
        let orig_view = GameMissionNodePlayerView::test_default();

        let mut buf = VSizedBuffer::new(orig_view.size_in_buffer());
        buf.push(&orig_view);
        let new_view = buf.pull::<GameMissionNodePlayerView>();

        assert_eq!(orig_view, new_view);
    }
}
