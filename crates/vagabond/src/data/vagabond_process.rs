use hall::core::PriorityType;

use crate::data::VagabondCard;

#[derive(Clone)]
pub struct VagabondProcess {
    pub card: VagabondCard,
    pub priority: PriorityType,
    pub local: bool,
}
