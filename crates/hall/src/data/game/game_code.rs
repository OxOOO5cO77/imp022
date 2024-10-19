use serde::{Deserialize, Serialize};
use shared_data::game::opcode::OpCode;
use crate::data::game::game_machine::GameMachineContext;

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct GameCode(pub OpCode);

impl GameCode {
    pub(crate) fn execute(&self, context: &mut GameMachineContext) -> bool {
        match self.0 {
            OpCode::NoOp => {}
            OpCode::ChangeFreeSpace(amount) => context.free_space = context.free_space.saturating_add_signed(amount),
            OpCode::ChangeThermalCapacity(amount) => context.thermal_capacity = context.thermal_capacity.saturating_add_signed(amount),
            OpCode::ChangeSystemHealth(amount) => context.system_health = context.system_health.saturating_add_signed(amount),
            OpCode::ChangeOpenPorts(amount) => context.open_ports = context.open_ports.saturating_add_signed(amount),
        }
        true
    }
}
