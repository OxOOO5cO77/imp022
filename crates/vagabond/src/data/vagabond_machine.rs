use crate::data::vagabond_process::VagabondProcess;
use hall::data::core::DelayType;
use hall::data::game::MachineValueType;

#[derive(Default, Clone)]
pub struct VagabondMachine {
    pub vitals: [MachineValueType; 4],
    pub queue: Vec<(VagabondProcess, DelayType)>,
    pub running: Vec<VagabondProcess>,
}
