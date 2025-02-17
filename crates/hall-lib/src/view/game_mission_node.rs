use shared_net::{Bufferable, SizedBuffer, SizedBufferError};

use crate::core::{AuthLevel, MissionNodeContent, MissionNodeIdType, MissionNodeKind, MissionNodeLinkDir, RemoteIdType};
use crate::view::game_actor::GameActorPlayerView;

#[derive(Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct MissionNodeLinkView {
    pub direction: MissionNodeLinkDir,
    pub target: MissionNodeIdType,
    pub min_level: AuthLevel,
    pub locked: bool,
}

#[derive(Default, Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameMissionNodePlayerView {
    pub id: MissionNodeIdType,
    pub kind: MissionNodeKind,
    pub links: Vec<MissionNodeLinkView>,
    pub content: Vec<MissionNodeContent>,
    pub remote: RemoteIdType,
    pub actors: Vec<GameActorPlayerView>,
}

type PackedMissionNodeType = u16;

const BITS_FOR_ID: PackedMissionNodeType = 8;
const BITS_FOR_KIND: PackedMissionNodeType = 5;

const SHIFT_FOR_ID: PackedMissionNodeType = 0;
const SHIFT_FOR_KIND: PackedMissionNodeType = SHIFT_FOR_ID + BITS_FOR_ID;

const MASK_FOR_ID: PackedMissionNodeType = (1 << BITS_FOR_ID) - 1;
const MASK_FOR_KIND: PackedMissionNodeType = (1 << BITS_FOR_KIND) - 1;

type PackedMissionNodeLinkType = u64;

pub const MAX_LINK_COUNT: usize = 4;

const BITS_FOR_LINK_DIR: PackedMissionNodeLinkType = 2;
const BITS_FOR_LINK_MIN_LEVEL: PackedMissionNodeLinkType = 3;
const BITS_FOR_LINK_TARGET: PackedMissionNodeLinkType = 8;
const BITS_FOR_LINK_LOCKED: PackedMissionNodeLinkType = 1;
const BITS_FOR_LINK_VALID: PackedMissionNodeLinkType = 1;

const BITS_FOR_LINK: PackedMissionNodeLinkType = BITS_FOR_LINK_DIR + BITS_FOR_LINK_MIN_LEVEL + BITS_FOR_LINK_TARGET + BITS_FOR_LINK_LOCKED + BITS_FOR_LINK_VALID;

const SHIFT_FOR_LINK_DIR: PackedMissionNodeLinkType = 0;
const SHIFT_FOR_LINK_MIN_LEVEL: PackedMissionNodeLinkType = SHIFT_FOR_LINK_DIR + BITS_FOR_LINK_DIR;
const SHIFT_FOR_LINK_TARGET: PackedMissionNodeLinkType = SHIFT_FOR_LINK_MIN_LEVEL + BITS_FOR_LINK_MIN_LEVEL;
const SHIFT_FOR_LINK_LOCKED: PackedMissionNodeLinkType = SHIFT_FOR_LINK_TARGET + BITS_FOR_LINK_TARGET;
const SHIFT_FOR_LINK_VALID: PackedMissionNodeLinkType = SHIFT_FOR_LINK_LOCKED + BITS_FOR_LINK_LOCKED;

const MASK_FOR_LINK_DIR: PackedMissionNodeLinkType = (1 << BITS_FOR_LINK_DIR) - 1;
const MASK_FOR_LINK_MIN_LEVEL: PackedMissionNodeLinkType = (1 << BITS_FOR_LINK_MIN_LEVEL) - 1;
const MASK_FOR_LINK_TARGET: PackedMissionNodeLinkType = (1 << BITS_FOR_LINK_TARGET) - 1;
const MASK_FOR_LINK_LOCKED: PackedMissionNodeLinkType = (1 << BITS_FOR_LINK_LOCKED) - 1;
const MASK_FOR_LINK: PackedMissionNodeLinkType = (1 << BITS_FOR_LINK) - 1;
const MASK_FOR_LINK_VALID: PackedMissionNodeLinkType = (1 << BITS_FOR_LINK_VALID) - 1;

pub const MAX_CONTENT_COUNT: usize = 4;
pub const MAX_ACTOR_COUNT: usize = 8;

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

    fn pack_link_min_level(min_level: AuthLevel) -> PackedMissionNodeLinkType {
        let min_level: u8 = min_level.into();
        min_level as PackedMissionNodeLinkType
    }
    fn unpack_link_min_level(packed: PackedMissionNodeLinkType) -> AuthLevel {
        (packed as u8).into()
    }

    fn pack_link(link: &MissionNodeLinkView) -> PackedMissionNodeLinkType {
        let mut packed = 0;
        packed |= Self::pack_link_dir(link.direction) << SHIFT_FOR_LINK_DIR;
        packed |= Self::pack_link_min_level(link.min_level) << SHIFT_FOR_LINK_MIN_LEVEL;
        packed |= (link.target as PackedMissionNodeLinkType) << SHIFT_FOR_LINK_TARGET;
        if link.locked {
            packed |= 1 << SHIFT_FOR_LINK_LOCKED;
        }
        packed |= 1 << SHIFT_FOR_LINK_VALID;
        packed
    }
    fn unpack_link(packed: PackedMissionNodeLinkType) -> (MissionNodeLinkView, bool) {
        let result = MissionNodeLinkView {
            direction: Self::unpack_link_dir((packed >> SHIFT_FOR_LINK_DIR) & MASK_FOR_LINK_DIR),
            target: ((packed >> SHIFT_FOR_LINK_TARGET) & MASK_FOR_LINK_TARGET) as MissionNodeIdType,
            min_level: Self::unpack_link_min_level((packed >> SHIFT_FOR_LINK_MIN_LEVEL) & MASK_FOR_LINK_MIN_LEVEL),
            locked: ((packed >> SHIFT_FOR_LINK_LOCKED) & MASK_FOR_LINK_LOCKED) != 0,
        };
        let valid = ((packed >> SHIFT_FOR_LINK_VALID) & MASK_FOR_LINK_VALID) != 0;
        (result, valid)
    }

    fn pack_links(&self) -> PackedMissionNodeLinkType {
        let mut packed = 0;
        for (i, link) in self.links.iter().enumerate() {
            packed |= Self::pack_link(link) << (i as PackedMissionNodeLinkType * BITS_FOR_LINK);
        }
        packed
    }
    fn unpack_links(packed: PackedMissionNodeLinkType) -> Vec<MissionNodeLinkView> {
        let mut links = Vec::new();
        for i in 0..MAX_LINK_COUNT as PackedMissionNodeLinkType {
            let (link, valid) = Self::unpack_link((packed >> (BITS_FOR_LINK * i)) & MASK_FOR_LINK);
            if valid {
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
    fn push_into(&self, buf: &mut SizedBuffer) -> Result<usize, SizedBufferError> {
        let mut pushed = 0;
        pushed += self.pack_mission().push_into(buf)?;
        pushed += self.pack_links().push_into(buf)?;
        pushed += self.content.push_into(buf)?;
        pushed += self.remote.push_into(buf)?;
        pushed += self.actors.push_into(buf)?;
        Ok(pushed)
    }

    fn pull_from(buf: &mut SizedBuffer) -> Result<Self, SizedBufferError> {
        let packed_mission = PackedMissionNodeType::pull_from(buf)?;
        let (id, kind) = Self::unpack_mission(packed_mission);
        let packed_links = PackedMissionNodeLinkType::pull_from(buf)?;
        let links = Self::unpack_links(packed_links);
        let content = <Vec<MissionNodeContent>>::pull_from(buf)?;
        let remote = RemoteIdType::pull_from(buf)?;
        let actors = <Vec<GameActorPlayerView>>::pull_from(buf)?;
        let result = Self {
            id,
            kind,
            links,
            content,
            remote,
            actors,
        };
        Ok(result)
    }

    fn size_in_buffer(&self) -> usize {
        size_of::<PackedMissionNodeType>() + size_of::<PackedMissionNodeLinkType>() + self.content.size_in_buffer() + self.remote.size_in_buffer() + self.actors.size_in_buffer()
    }
}

#[cfg(test)]
impl GameMissionNodePlayerView {
    pub fn test_default() -> Self {
        Self {
            id: 123,
            kind: MissionNodeKind::Frontend,
            links: vec![
                MissionNodeLinkView {
                    direction: MissionNodeLinkDir::North,
                    target: 0,
                    min_level: AuthLevel::Admin,
                    locked: true,
                },
                MissionNodeLinkView {
                    direction: MissionNodeLinkDir::East,
                    target: 3,
                    min_level: AuthLevel::Anonymous,
                    locked: false,
                },
                MissionNodeLinkView {
                    direction: MissionNodeLinkDir::South,
                    target: 234,
                    min_level: AuthLevel::User,
                    locked: true,
                },
            ],
            content: Vec::new(),
            remote: 12345678901234567890,
            actors: vec![GameActorPlayerView::test_default(), GameActorPlayerView::test_default(), GameActorPlayerView::test_default(), GameActorPlayerView::test_default()],
        }
    }
}

#[cfg(test)]
mod test {
    use crate::view::GameMissionNodePlayerView;
    use shared_net::{SizedBuffer, SizedBufferError};

    #[test]
    fn test_game_mission_node_player_view() -> Result<(), SizedBufferError> {
        let orig_view = GameMissionNodePlayerView::test_default();

        let mut buf = SizedBuffer::from(&orig_view)?;
        let new_view = buf.pull::<GameMissionNodePlayerView>()?;

        assert_eq!(orig_view, new_view);
        Ok(())
    }
}
