use crate::data::VagabondCard;
use hall::data::core::PriorityType;

#[derive(Clone)]
pub struct VagabondProcess {
    pub card: VagabondCard,
    pub priority: PriorityType,
    pub local: bool,
}
