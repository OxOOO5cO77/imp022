use hall::core::{DelayType, MachineValueType};

use crate::data::vagabond_process::VagabondProcess;

#[derive(Default, Clone)]
pub struct VagabondMachine {
    pub vitals: [MachineValueType; 4],
    pub queue: Vec<(VagabondProcess, DelayType)>,
    pub running: Vec<VagabondProcess>,
}
