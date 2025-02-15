use hall_lib::core::{AttributeArrays, PriorityType};

use crate::data::VagabondCard;

#[derive(Clone)]
pub struct VagabondProcess {
    pub card: VagabondCard,
    pub priority: PriorityType,
    pub local: bool,
    pub attributes: AttributeArrays,
}
