use crate::data::VagabondCard;
use shared_data::card::PriorityType;

#[derive(Clone)]
pub struct VagabondProcess {
    pub card: VagabondCard,
    pub priority: PriorityType,
    pub local: bool,
}
