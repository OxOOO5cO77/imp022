use serde::Deserialize;
use crate::mission::node::MissionNode;
use crate::mission::objective::MissionObjective;

mod node;
mod objective;

#[derive(Clone, Deserialize)]
pub struct Mission {
    node: Vec<MissionNode>,
    objective: Vec<MissionObjective>,
}
