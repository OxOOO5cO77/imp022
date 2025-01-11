use crate::core::{MissionNodeContent, MissionNodeIdType, MissionNodeKind, MissionNodeLink, MissionNodeLinkDir, MissionNodeLinkState, RemoteIdType};
use shared_net::{Bufferable, VSizedBuffer};

#[derive(Default, Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameMissionNodePlayerView {
    pub id: MissionNodeIdType,
    pub kind: MissionNodeKind,
    pub links: Vec<MissionNodeLink>,
    pub content: Vec<MissionNodeContent>,
    pub remote: RemoteIdType,
}

type PackedMissionNodeType = u64;

const MAX_LINK_COUNT: PackedMissionNodeType = 4;

const BITS_FOR_ID: PackedMissionNodeType = 8;
const BITS_FOR_KIND: PackedMissionNodeType = 5;
const BITS_FOR_LINK_DIR: PackedMissionNodeType = 2;
const BITS_FOR_LINK_STATE: PackedMissionNodeType = 1;
const BITS_FOR_LINK_TARGET: PackedMissionNodeType = 8;
const BITS_FOR_LINK: PackedMissionNodeType = BITS_FOR_LINK_DIR + BITS_FOR_LINK_STATE + BITS_FOR_LINK_TARGET;
const BITS_FOR_LINKS: PackedMissionNodeType = BITS_FOR_LINK * 4;

const SHIFT_FOR_ID: PackedMissionNodeType = 0;
const SHIFT_FOR_KIND: PackedMissionNodeType = SHIFT_FOR_ID + BITS_FOR_ID;
const SHIFT_FOR_LINKS: PackedMissionNodeType = SHIFT_FOR_KIND + BITS_FOR_KIND;
const SHIFT_FOR_LINK_DIR: PackedMissionNodeType = 0;
const SHIFT_FOR_LINK_STATE: PackedMissionNodeType = SHIFT_FOR_LINK_DIR + BITS_FOR_LINK_DIR;
const SHIFT_FOR_LINK_TARGET: PackedMissionNodeType = SHIFT_FOR_LINK_STATE + BITS_FOR_LINK_STATE;

const MASK_FOR_ID: PackedMissionNodeType = (1 << BITS_FOR_ID) - 1;
const MASK_FOR_KIND: PackedMissionNodeType = (1 << BITS_FOR_KIND) - 1;
const MASK_FOR_LINK_DIR: PackedMissionNodeType = (1 << BITS_FOR_LINK_DIR) - 1;
const MASK_FOR_LINK_STATE: PackedMissionNodeType = (1 << BITS_FOR_LINK_STATE) - 1;
const MASK_FOR_LINK_TARGET: PackedMissionNodeType = (1 << BITS_FOR_LINK_TARGET) - 1;
const MASK_FOR_LINKS: PackedMissionNodeType = (1 << BITS_FOR_LINKS) - 1;
const MASK_FOR_LINK: PackedMissionNodeType = (1 << BITS_FOR_LINK) - 1;

impl GameMissionNodePlayerView {
    fn pack_kind(kind: MissionNodeKind) -> PackedMissionNodeType {
        let kind: u8 = kind.into();
        kind as PackedMissionNodeType
    }
    fn unpack_kind(packed: PackedMissionNodeType) -> MissionNodeKind {
        (packed as u8).into()
    }

    fn pack_link_dir(dir: MissionNodeLinkDir) -> PackedMissionNodeType {
        let dir: u8 = dir.into();
        dir as PackedMissionNodeType
    }
    fn unpack_link_dir(packed: PackedMissionNodeType) -> MissionNodeLinkDir {
        (packed as u8).into()
    }

    fn pack_link_state(state: MissionNodeLinkState) -> PackedMissionNodeType {
        let state: u8 = state.into();
        state as PackedMissionNodeType
    }
    fn unpack_link_state(packed: PackedMissionNodeType) -> MissionNodeLinkState {
        (packed as u8).into()
    }

    fn pack_link(link: &MissionNodeLink) -> PackedMissionNodeType {
        let mut packed = 0;
        packed |= Self::pack_link_dir(link.direction) << SHIFT_FOR_LINK_DIR;
        packed |= Self::pack_link_state(link.state) << SHIFT_FOR_LINK_STATE;
        packed |= (link.target as PackedMissionNodeType) << SHIFT_FOR_LINK_TARGET;
        packed
    }
    fn unpack_link(packed: PackedMissionNodeType) -> MissionNodeLink {
        MissionNodeLink {
            direction: Self::unpack_link_dir((packed >> SHIFT_FOR_LINK_DIR) & MASK_FOR_LINK_DIR),
            target: ((packed >> SHIFT_FOR_LINK_TARGET) & MASK_FOR_LINK_TARGET) as MissionNodeIdType,
            state: Self::unpack_link_state((packed >> SHIFT_FOR_LINK_STATE) & MASK_FOR_LINK_STATE),
        }
    }

    fn pack_links(links: &[MissionNodeLink]) -> PackedMissionNodeType {
        let mut packed = 0;
        for (i, link) in links.iter().enumerate() {
            packed |= Self::pack_link(link) << (i as PackedMissionNodeType * BITS_FOR_LINK);
        }
        packed
    }
    fn unpack_links(packed: PackedMissionNodeType) -> Vec<MissionNodeLink> {
        let mut links = Vec::new();
        for i in 0..MAX_LINK_COUNT {
            let link = Self::unpack_link((packed >> (BITS_FOR_LINK * i)) & MASK_FOR_LINK);
            if link.target != 0 {
                links.push(link);
            }
        }
        links
    }

    fn pack_mission(&self) -> PackedMissionNodeType {
        let packed_id = (self.id as PackedMissionNodeType) << SHIFT_FOR_ID;
        let packed_kind = Self::pack_kind(self.kind) << SHIFT_FOR_KIND;
        let packed_links = Self::pack_links(&self.links) << SHIFT_FOR_LINKS;
        packed_id | packed_kind | packed_links
    }

    fn unpack_mission(packed: PackedMissionNodeType) -> (MissionNodeIdType, MissionNodeKind, Vec<MissionNodeLink>) {
        let id = ((packed >> SHIFT_FOR_ID) & MASK_FOR_ID) as MissionNodeIdType;
        let kind = Self::unpack_kind((packed >> SHIFT_FOR_KIND) & MASK_FOR_KIND);
        let links = Self::unpack_links((packed >> SHIFT_FOR_LINKS) & MASK_FOR_LINKS);
        (id, kind, links)
    }
}

impl Bufferable for GameMissionNodePlayerView {
    fn push_into(&self, buf: &mut VSizedBuffer) {
        let packed = self.pack_mission();
        packed.push_into(buf);
        self.content.push_into(buf);
        self.remote.push_into(buf);
    }

    fn pull_from(buf: &mut VSizedBuffer) -> Self {
        let packed = PackedMissionNodeType::pull_from(buf);
        let (id, kind, links) = Self::unpack_mission(packed);
        let content = <Vec<MissionNodeContent>>::pull_from(buf);
        let remote = RemoteIdType::pull_from(buf);
        Self {
            id,
            kind,
            links,
            content,
            remote,
        }
    }

    fn size_in_buffer(&self) -> usize {
        size_of::<PackedMissionNodeType>() + self.content.size_in_buffer() + self.remote.size_in_buffer()
    }
}

#[cfg(test)]
impl GameMissionNodePlayerView {
    pub fn test_default() -> Self {
        Self {
            id: 123,
            kind: MissionNodeKind::Frontend,
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
            content: Vec::new(),
            remote: 12345678901234567890,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::view::GameMissionNodePlayerView;
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
