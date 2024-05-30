use serde::Deserialize;
use crate::mission::mission_node::MissionNode;
use crate::mission::mission_objective::MissionObjective;

mod mission_node;
mod mission_objective;

#[derive(Clone, Deserialize)]
pub struct Mission {
    node: Vec<MissionNode>,
    objective: Vec<MissionObjective>,
}
