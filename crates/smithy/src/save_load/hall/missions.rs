use crate::data::{DbMission, DbMissionNode};
use crate::save_load::save_data_single;
use hall::data::hall::{HallMission, HallMissionNode};
use std::io::Error;

fn make_hall_mission_node(mission_node_instance: &DbMissionNode) -> HallMissionNode {
    HallMissionNode {
        id: mission_node_instance.node_id,
        kind: mission_node_instance.kind,
        state: mission_node_instance.state,
        links: mission_node_instance.links.clone(),
        content: vec![],
    }
}

fn make_hall_mission(mission_instance: &DbMission) -> HallMission {
    let mut node = mission_instance.node.iter().map(make_hall_mission_node).collect::<Vec<_>>();
    node.sort_by_key(|n| n.id);
    HallMission {
        id: mission_instance.mission_id,
        node,
        objective: vec![],
    }
}

pub(crate) fn output_missions_for_hall(missions: &[DbMission]) -> Result<(), Error> {
    let mut hall_missions = missions.iter().map(make_hall_mission).collect::<Vec<_>>();
    hall_missions.sort_by_key(|m| m.id);
    save_data_single(hall_missions, "output/hall_missions.ron")
}
