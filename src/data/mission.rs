use serde::Deserialize;
use crate::data::mission::mission_node::MissionNode;
use crate::data::mission::mission_objective::MissionObjective;

mod mission_node;
mod mission_objective;

#[derive(Clone, Deserialize)]
pub(crate) struct Mission {
    node: Vec<MissionNode>,
    objective: Vec<MissionObjective>,
}
