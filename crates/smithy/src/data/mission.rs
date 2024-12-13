use shared_data::mission::{MissionIdType, MissionNodeIdType, MissionNodeKind, MissionNodeLink, MissionNodeLinkDir, MissionNodeLinkState, MissionNodeState};
use sqlx::postgres::PgRow;
use sqlx::{Pool, Postgres, Row};

#[derive(sqlx::Type)]
#[sqlx(type_name = "type_missionnode")]
enum DbMissionNodeKind {
    AccessPoint,
    Backend,
    Control,
    Database,
    Engine,
    Frontend,
    Gateway,
    Hardware,
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "type_missionnodestate")]
enum DbMissionNodeState {
    Unknown,
    Known,
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "type_missionlinkstate")]
enum DbMissionLinkState {
    Closed,
    Open,
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "type_missionlink")]
struct DbMissionLink {
    target: i32,
    state: DbMissionLinkState,
}

pub(crate) struct DbMissionNode {
    pub(crate) mission_id: MissionIdType,
    pub(crate) node_id: MissionNodeIdType,
    pub(crate) kind: MissionNodeKind,
    pub(crate) state: MissionNodeState,
    pub(crate) links: Vec<MissionNodeLink>,
}

pub(crate) struct DbMission {
    pub(crate) mission_id: MissionIdType,
    pub(crate) node: Vec<DbMissionNode>,
}

impl From<DbMissionNodeKind> for MissionNodeKind {
    fn from(value: DbMissionNodeKind) -> Self {
        match value {
            DbMissionNodeKind::AccessPoint => MissionNodeKind::AccessPoint,
            DbMissionNodeKind::Backend => MissionNodeKind::Backend,
            DbMissionNodeKind::Control => MissionNodeKind::Control,
            DbMissionNodeKind::Database => MissionNodeKind::Database,
            DbMissionNodeKind::Engine => MissionNodeKind::Engine,
            DbMissionNodeKind::Frontend => MissionNodeKind::Frontend,
            DbMissionNodeKind::Gateway => MissionNodeKind::Gateway,
            DbMissionNodeKind::Hardware => MissionNodeKind::Hardware,
        }
    }
}

impl From<DbMissionLinkState> for MissionNodeLinkState {
    fn from(value: DbMissionLinkState) -> Self {
        match value {
            DbMissionLinkState::Closed => MissionNodeLinkState::Closed,
            DbMissionLinkState::Open => MissionNodeLinkState::Open,
        }
    }
}

impl From<DbMissionNodeState> for MissionNodeState {
    fn from(value: DbMissionNodeState) -> MissionNodeState {
        match value {
            DbMissionNodeState::Unknown => MissionNodeState::Unknown,
            DbMissionNodeState::Known => MissionNodeState::Known,
        }
    }
}

fn make_link(direction: MissionNodeLinkDir, db_link: DbMissionLink) -> MissionNodeLink {
    MissionNodeLink {
        direction,
        target: db_link.target as MissionNodeIdType,
        state: db_link.state.into(),
    }
}

fn make_links(north: Option<DbMissionLink>, east: Option<DbMissionLink>, south: Option<DbMissionLink>, west: Option<DbMissionLink>) -> Vec<MissionNodeLink> {
    let mut result = Vec::new();

    if let Some(north) = north {
        result.push(make_link(MissionNodeLinkDir::North, north));
    }
    if let Some(east) = east {
        result.push(make_link(MissionNodeLinkDir::East, east));
    }
    if let Some(south) = south {
        result.push(make_link(MissionNodeLinkDir::North, south));
    }
    if let Some(west) = west {
        result.push(make_link(MissionNodeLinkDir::North, west));
    }

    result
}

fn row_to_mission_node(row: &PgRow) -> DbMissionNode {
    DbMissionNode {
        mission_id: row.get::<i32, _>("mission_id") as MissionIdType,
        node_id: row.get::<i32, _>("node_id") as MissionNodeIdType,
        kind: row.get::<DbMissionNodeKind, _>("kind").into(),
        state: row.get::<DbMissionNodeState, _>("state").into(),
        links: make_links(row.get::<Option<DbMissionLink>, _>("north"), row.get::<Option<DbMissionLink>, _>("east"), row.get::<Option<DbMissionLink>, _>("south"), row.get::<Option<DbMissionLink>, _>("west")),
    }
}

pub(crate) async fn process_mission(pool: &Pool<Postgres>) -> Result<Vec<DbMission>, sqlx::Error> {
    let rows = sqlx::query("SELECT * FROM mission").fetch_all(pool).await?;
    let nodes = rows.iter().map(row_to_mission_node).collect::<Vec<DbMissionNode>>();

    let mut missions = Vec::<DbMission>::new();

    for node in nodes {
        if let Some(existing) = missions.iter_mut().find(|m| m.mission_id == node.mission_id) {
            existing.node.push(node);
        } else {
            missions.push(DbMission {
                mission_id: node.mission_id,
                node: vec![node],
            })
        }
    }

    Ok(missions)
}
