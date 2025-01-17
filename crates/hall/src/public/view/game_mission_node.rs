use crate::core::{ActorIdType, MissionNodeContent, MissionNodeIdType, MissionNodeKind, MissionNodeLink, MissionNodeLinkDamageType, MissionNodeLinkDir, MissionNodeLinkState, RemoteIdType};
use shared_net::{Bufferable, VSizedBuffer};

#[derive(Default, Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameMissionNodePlayerView {
    pub id: MissionNodeIdType,
    pub kind: MissionNodeKind,
    pub links: Vec<MissionNodeLink>,
    pub content: Vec<MissionNodeContent>,
    pub remote: RemoteIdType,
    pub users: Vec<ActorIdType>,
}

type PackedMissionNodeType = u16;

const BITS_FOR_ID: PackedMissionNodeType = 8;
const BITS_FOR_KIND: PackedMissionNodeType = 5;

const SHIFT_FOR_ID: PackedMissionNodeType = 0;
const SHIFT_FOR_KIND: PackedMissionNodeType = SHIFT_FOR_ID + BITS_FOR_ID;

const MASK_FOR_ID: PackedMissionNodeType = (1 << BITS_FOR_ID) - 1;
const MASK_FOR_KIND: PackedMissionNodeType = (1 << BITS_FOR_KIND) - 1;

type PackedMissionNodeLinkType = u64;

pub const MAX_LINK_DAMAGE: MissionNodeLinkDamageType = 10;
pub const MAX_LINK_COUNT: usize = 4;

const BITS_FOR_LINK_DIR: PackedMissionNodeLinkType = 2;
const BITS_FOR_LINK_STATE: PackedMissionNodeLinkType = 1;
const BITS_FOR_LINK_TARGET: PackedMissionNodeLinkType = 8;
const BITS_FOR_LINK_DAMAGE: PackedMissionNodeLinkType = 4;
const BITS_FOR_LINK: PackedMissionNodeLinkType = BITS_FOR_LINK_DIR + BITS_FOR_LINK_STATE + BITS_FOR_LINK_TARGET + BITS_FOR_LINK_DAMAGE;

const SHIFT_FOR_LINK_DIR: PackedMissionNodeLinkType = 0;
const SHIFT_FOR_LINK_STATE: PackedMissionNodeLinkType = SHIFT_FOR_LINK_DIR + BITS_FOR_LINK_DIR;
const SHIFT_FOR_LINK_TARGET: PackedMissionNodeLinkType = SHIFT_FOR_LINK_STATE + BITS_FOR_LINK_STATE;
const SHIFT_FOR_LINK_DAMAGE: PackedMissionNodeLinkType = SHIFT_FOR_LINK_TARGET + BITS_FOR_LINK_TARGET;

const MASK_FOR_LINK_DIR: PackedMissionNodeLinkType = (1 << BITS_FOR_LINK_DIR) - 1;
const MASK_FOR_LINK_STATE: PackedMissionNodeLinkType = (1 << BITS_FOR_LINK_STATE) - 1;
const MASK_FOR_LINK_TARGET: PackedMissionNodeLinkType = (1 << BITS_FOR_LINK_TARGET) - 1;
const MASK_FOR_LINK_DAMAGE: PackedMissionNodeLinkType = (1 << BITS_FOR_LINK_DAMAGE) - 1;
const MASK_FOR_LINK: PackedMissionNodeLinkType = (1 << BITS_FOR_LINK) - 1;

pub const MAX_CONTENT_COUNT: usize = 4;
pub const MAX_USER_COUNT: usize = 8;

impl GameMissionNodePlayerView {
    fn pack_kind(kind: MissionNodeKind) -> PackedMissionNodeType {
        let kind: u8 = kind.into();
        kind as PackedMissionNodeType
    }
    fn unpack_kind(packed: PackedMissionNodeType) -> MissionNodeKind {
        (packed as u8).into()
    }

    fn pack_link_dir(dir: MissionNodeLinkDir) -> PackedMissionNodeLinkType {
        let dir: u8 = dir.into();
        dir as PackedMissionNodeLinkType
    }
    fn unpack_link_dir(packed: PackedMissionNodeLinkType) -> MissionNodeLinkDir {
        (packed as u8).into()
    }

    fn pack_link_state(state: MissionNodeLinkState) -> PackedMissionNodeLinkType {
        let state: u8 = state.into();
        state as PackedMissionNodeLinkType
    }
    fn unpack_link_state(packed: PackedMissionNodeLinkType) -> MissionNodeLinkState {
        (packed as u8).into()
    }

    fn pack_link(link: &MissionNodeLink) -> PackedMissionNodeLinkType {
        let mut packed = 0;
        packed |= Self::pack_link_dir(link.direction) << SHIFT_FOR_LINK_DIR;
        packed |= Self::pack_link_state(link.state) << SHIFT_FOR_LINK_STATE;
        packed |= (link.target as PackedMissionNodeLinkType) << SHIFT_FOR_LINK_TARGET;
        packed |= (link.damage as PackedMissionNodeLinkType) << SHIFT_FOR_LINK_DAMAGE;
        packed
    }
    fn unpack_link(packed: PackedMissionNodeLinkType) -> MissionNodeLink {
        MissionNodeLink {
            direction: Self::unpack_link_dir((packed >> SHIFT_FOR_LINK_DIR) & MASK_FOR_LINK_DIR),
            target: ((packed >> SHIFT_FOR_LINK_TARGET) & MASK_FOR_LINK_TARGET) as MissionNodeIdType,
            state: Self::unpack_link_state((packed >> SHIFT_FOR_LINK_STATE) & MASK_FOR_LINK_STATE),
            damage: ((packed >> SHIFT_FOR_LINK_DAMAGE) & MASK_FOR_LINK_DAMAGE) as MissionNodeLinkDamageType,
        }
    }

    fn pack_links(&self) -> PackedMissionNodeLinkType {
        let mut packed = 0;
        for (i, link) in self.links.iter().enumerate() {
            packed |= Self::pack_link(link) << (i as PackedMissionNodeLinkType * BITS_FOR_LINK);
        }
        packed
    }
    fn unpack_links(packed: PackedMissionNodeLinkType) -> Vec<MissionNodeLink> {
        let mut links = Vec::new();
        for i in 0..MAX_LINK_COUNT as PackedMissionNodeLinkType {
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
        packed_id | packed_kind
    }
    fn unpack_mission(packed: PackedMissionNodeType) -> (MissionNodeIdType, MissionNodeKind) {
        let id = ((packed >> SHIFT_FOR_ID) & MASK_FOR_ID) as MissionNodeIdType;
        let kind = Self::unpack_kind((packed >> SHIFT_FOR_KIND) & MASK_FOR_KIND);
        (id, kind)
    }
}

impl Bufferable for GameMissionNodePlayerView {
    fn push_into(&self, buf: &mut VSizedBuffer) {
        let packed_mission = self.pack_mission();
        packed_mission.push_into(buf);
        let packed_links = self.pack_links();
        packed_links.push_into(buf);
        self.content.push_into(buf);
        self.remote.push_into(buf);
        self.users.push_into(buf);
    }

    fn pull_from(buf: &mut VSizedBuffer) -> Self {
        let packed_mission = PackedMissionNodeType::pull_from(buf);
        let (id, kind) = Self::unpack_mission(packed_mission);
        let packed_links = PackedMissionNodeLinkType::pull_from(buf);
        let links = Self::unpack_links(packed_links);
        let content = <Vec<MissionNodeContent>>::pull_from(buf);
        let remote = RemoteIdType::pull_from(buf);
        let users = <Vec<ActorIdType>>::pull_from(buf);
        Self {
            id,
            kind,
            links,
            content,
            remote,
            users,
        }
    }

    fn size_in_buffer(&self) -> usize {
        size_of::<PackedMissionNodeType>() + size_of::<PackedMissionNodeLinkType>() + self.content.size_in_buffer() + self.remote.size_in_buffer() + self.users.size_in_buffer()
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
                    damage: 10,
                },
                MissionNodeLink {
                    direction: MissionNodeLinkDir::East,
                    target: 3,
                    state: MissionNodeLinkState::Open,
                    damage: 0,
                },
                MissionNodeLink {
                    direction: MissionNodeLinkDir::South,
                    target: 234,
                    state: MissionNodeLinkState::Open,
                    damage: 0,
                },
                MissionNodeLink {
                    direction: MissionNodeLinkDir::West,
                    target: 1,
                    state: MissionNodeLinkState::Closed,
                    damage: 5,
                },
            ],
            content: Vec::new(),
            remote: 12345678901234567890,
            users: Vec::new(),
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
