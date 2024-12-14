use crate::data::vagabond_process::VagabondProcess;
use hall::data::game::MachineValueType;
use shared_data::card::DelayType;

#[derive(Default, Clone)]
pub struct VagabondMachine {
    pub vitals: [MachineValueType; 4],
    pub queue: Vec<(VagabondProcess, DelayType)>,
    pub running: Vec<VagabondProcess>,
}
