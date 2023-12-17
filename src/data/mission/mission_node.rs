use serde::Deserialize;

#[derive(Clone, Deserialize)]
enum MissionNodeKind {
    AccessPoint,
    Backend,
    Control,
    Database,
    Executor,
    Frontend,
    Gateway,
    Hardware,
}

#[derive(Clone, Deserialize)]
enum MissionNodeState {
    Unknown,
}

#[derive(Clone, Deserialize)]
enum MissionNodeLinkDir {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, Deserialize)]
enum MissionNodeLinkState {
    Closed,
}

#[derive(Clone, Deserialize)]
struct MissionNodeLink {
    direction: MissionNodeLinkDir,
    state: MissionNodeLinkState,
}

#[derive(Clone, Deserialize)]
struct MissionNodeContent {}

#[derive(Clone, Deserialize)]
pub(crate) struct MissionNode {
    kind: MissionNodeKind,
    state: MissionNodeState,
    link: Vec<MissionNodeLink>,
    content: Vec<MissionNodeContent>,
}
