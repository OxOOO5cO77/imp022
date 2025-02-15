use hall_lib::core::{DelayType, RuntimeAmountType};

use crate::data::vagabond_process::VagabondProcess;

#[derive(Default, Clone)]
pub struct VagabondMachine {
    pub vitals: [RuntimeAmountType; 4],
    pub queue: Vec<(VagabondProcess, DelayType)>,
    pub running: Vec<VagabondProcess>,
}
