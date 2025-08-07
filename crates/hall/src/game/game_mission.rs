use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};

use hall_lib::core::{AuthLevel, GeneralType, MissionIdType, MissionNodeIdType, MissionNodeKind, MissionNodeLink, MissionNodeLinkDir, MissionNodeState, SpecificType};
use hall_lib::view::{GameMissionObjectivePlayerView, GameMissionPlayerView};

use crate::game::game_state::ActorMapType;
use crate::game::{GameMissionNode, GameMissionObjective, GameUserMissionState};

#[derive(Default)]
pub(crate) struct GameMission {
    pub(crate) id: MissionIdType,
    pub(crate) institution: (GeneralType, SpecificType),
    pub(crate) node: Vec<GameMissionNode>,
    pub(crate) objective: Vec<GameMissionObjective>,
}

const MAP_SIZE: usize = 8;
type MapType = [MapNode; MAP_SIZE * MAP_SIZE];
const MAP_MAX: usize = MAP_SIZE - 1;

trait MapTypeExt {
    fn at(&self, coord: &MapCoord) -> &MapNode;
    fn at_mut(&mut self, coord: &MapCoord) -> &mut MapNode;
}

impl MapTypeExt for MapType {
    fn at(&self, coord: &MapCoord) -> &MapNode {
        &self[coord.as_ordinal() as usize]
    }
    fn at_mut(&mut self, coord: &MapCoord) -> &mut MapNode {
        &mut self[coord.as_ordinal() as usize]
    }
}

#[derive(Default, Clone, Copy)]
struct MapNode {
    id: MissionNodeIdType,
    exit: [bool; 4],
    distance: Option<u16>,
}

#[derive(Default, Clone, Copy)]
struct MapCoord {
    x: usize,
    y: usize,
}

impl MapCoord {
    fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
        }
    }

    fn adj_x(&self, x: isize) -> Self {
        Self {
            x: self.x.saturating_add_signed(x),
            y: self.y,
        }
    }

    fn adj_y(&self, y: isize) -> Self {
        Self {
            x: self.x,
            y: self.y.saturating_add_signed(y),
        }
    }

    fn as_ordinal(&self) -> MissionNodeIdType {
        (self.y * MAP_SIZE + self.x) as MissionNodeIdType
    }

    fn from_ordinal(ordinal: usize) -> Self {
        Self {
            x: ordinal % MAP_SIZE,
            y: ordinal / MAP_SIZE,
        }
    }
}

enum MapDir {
    North,
    East,
    South,
    West,
}

impl From<&MapDir> for MissionNodeLinkDir {
    fn from(value: &MapDir) -> Self {
        match value {
            MapDir::North => MissionNodeLinkDir::North,
            MapDir::East => MissionNodeLinkDir::East,
            MapDir::South => MissionNodeLinkDir::South,
            MapDir::West => MissionNodeLinkDir::West,
        }
    }
}

const DIRS: [MapDir; 4] = [MapDir::North, MapDir::East, MapDir::South, MapDir::West];
impl MapDir {
    fn value(&self) -> usize {
        match self {
            MapDir::North => 0,
            MapDir::East => 1,
            MapDir::South => 2,
            MapDir::West => 3,
        }
    }

    fn valid(&self, coord: &MapCoord) -> bool {
        match self {
            MapDir::North => coord.y != 0,
            MapDir::East => coord.x != MAP_MAX,
            MapDir::South => coord.y != MAP_MAX,
            MapDir::West => coord.x != 0,
        }
    }

    fn target(&self, coord: &MapCoord) -> Option<MapCoord> {
        if self.valid(coord) {
            let result = match self {
                MapDir::North => coord.adj_y(-1),
                MapDir::East => coord.adj_x(1),
                MapDir::South => coord.adj_y(1),
                MapDir::West => coord.adj_x(-1),
            };
            Some(result)
        } else {
            None
        }
    }

    fn opposite(&self) -> Self {
        match self {
            MapDir::North => MapDir::South,
            MapDir::East => MapDir::West,
            MapDir::South => MapDir::North,
            MapDir::West => MapDir::East,
        }
    }
}

impl GameMission {
    fn make_exit(map: &mut MapType, coord: &MapCoord, dir: &MapDir, force: bool, set: u16, rng: &mut impl Rng) -> Option<MapCoord> {
        let idx = dir.value();
        let target = dir.target(coord);
        let door = target.as_ref().map(|t| force || (map.at(t).distance != Some(set) && rng.random_bool(0.5))).unwrap_or_default();

        map.at_mut(coord).exit[idx] = door || map.at(coord).exit[idx];
        if door {
            target
        } else {
            None
        }
    }

    fn make_exits(map: &mut MapType, coord: &MapCoord, set: u16, force: bool, rng: &mut impl Rng) -> Vec<(MapCoord, u16, bool)> {
        map.at_mut(coord).distance = Some(set);
        map.at_mut(coord).id = coord.as_ordinal();

        let mut ret = Vec::new();

        for dir in DIRS.iter() {
            if let Some(target) = Self::make_exit(map, coord, dir, force, set, rng) {
                Self::make_exit(map, &target, &dir.opposite(), true, set, rng);

                ret.push((target, set, false));
            }
        }

        ret
    }

    fn set_distance(map: &mut MapType, coord: &MapCoord, distance: u16) -> bool {
        if let Some(cur) = map.at(coord).distance {
            map.at_mut(coord).distance = Some(cur.min(distance));
            cur > distance
        } else {
            false
        }
    }

    fn generate_map(rng: &mut impl Rng) -> MapType {
        let mut map = [MapNode::default(); MAP_SIZE * MAP_SIZE];

        let mut start_points = vec![
            //
            (MapCoord::new(0, 0), 1, true),
            (MapCoord::new(MAP_MAX, 0), 2, true),
            (MapCoord::new(0, MAP_MAX), 3, true),
            (MapCoord::new(MAP_MAX, MAP_MAX), 4, true),
        ];

        while !start_points.is_empty() {
            if let Some((coord, set, force)) = start_points.pop()
                && map.at(&coord).distance.is_none()
            {
                start_points.append(&mut Self::make_exits(&mut map, &coord, set, force, rng));
            }
        }

        map
    }

    fn fill_map_dist(map: &mut MapType) {
        for node in map.iter_mut() {
            if node.distance.is_some() {
                node.distance = Some(999);
            }
        }

        let dist_points = vec![
            //
            MapCoord::new(0, 0),
            MapCoord::new(MAP_MAX, 0),
            MapCoord::new(0, MAP_MAX),
            MapCoord::new(MAP_MAX, MAP_MAX),
        ];

        for coord in &dist_points {
            let mut travel = vec![(Some(*coord), 0)];
            while !travel.is_empty() {
                if let Some((Some(target), distance)) = travel.pop()
                    && Self::set_distance(map, &target, distance)
                {
                    for dir in &DIRS {
                        if map.at(&target).exit[dir.value()] {
                            travel.push((dir.target(&target), distance + 1));
                        }
                    }
                }
            }
        }
    }

    fn nodes_from_map(map: MapType) -> Vec<GameMissionNode> {
        let mut result = Vec::new();

        for (index, node) in map.iter().enumerate() {
            if let Some(distance) = node.distance {
                let kind = match distance {
                    0 => MissionNodeKind::AccessPoint,
                    1 => MissionNodeKind::Gateway,
                    2 => MissionNodeKind::Frontend,
                    3 => MissionNodeKind::Backend,
                    4 => MissionNodeKind::Hardware,
                    5 => MissionNodeKind::Control,
                    6 => MissionNodeKind::Engine,
                    _ => MissionNodeKind::Database,
                };

                let node = GameMissionNode {
                    id: node.id,
                    kind,
                    initial_state: if kind == MissionNodeKind::AccessPoint {
                        MissionNodeState::Known
                    } else {
                        MissionNodeState::Unknown
                    },
                    links: Self::links_from_exits(node.exit, MapCoord::from_ordinal(index), map),
                    content: vec![],
                    remote: 0,
                    actors: vec![],
                };

                result.push(node);
            }
        }

        result
    }

    fn links_from_exits(exits: [bool; 4], coord: MapCoord, map: MapType) -> Vec<MissionNodeLink> {
        let mut result = Vec::new();

        for (i, dir) in DIRS.iter().enumerate() {
            if exits[i] {
                let link = MissionNodeLink {
                    direction: dir.into(),
                    target: map.at(&dir.target(&coord).unwrap()).id,
                    min_level: AuthLevel::Guest,
                };
                result.push(link);
            }
        }

        result
    }

    pub(crate) fn generate(institution: (GeneralType, SpecificType), game_id: u64) -> GameMission {
        let mut rng: StdRng = SeedableRng::seed_from_u64(game_id);

        let mut map = Self::generate_map(&mut rng);
        Self::fill_map_dist(&mut map);
        let node = Self::nodes_from_map(map);

        GameMission {
            id: game_id,
            institution,
            node,
            objective: vec![],
        }
    }
}

impl GameMission {
    pub(crate) fn get_node(&self, node: MissionNodeIdType) -> Option<&GameMissionNode> {
        self.node.iter().find(|n| n.id == node)
    }

    pub(crate) fn get_node_mut(&mut self, node: MissionNodeIdType) -> Option<&mut GameMissionNode> {
        self.node.iter_mut().find(|n| n.id == node)
    }

    pub(crate) fn to_player_view(&self, mission_state: &GameUserMissionState, actors: &ActorMapType) -> GameMissionPlayerView {
        let id = self.id;
        let institution = self.institution;
        let current_node = mission_state.current();
        let auth_level = mission_state.max_auth_level();
        let node_map = mission_state.known_nodes.iter().filter_map(|id| self.get_node(*id)).map(|node| node.to_player_view(auth_level, actors)).collect();
        let tokens = mission_state.tokens.clone();
        let objective = self.objective.iter().map(GameMissionObjectivePlayerView::from).collect();

        GameMissionPlayerView {
            id,
            institution,
            current_node,
            node_map,
            tokens,
            objective,
        }
    }
}
