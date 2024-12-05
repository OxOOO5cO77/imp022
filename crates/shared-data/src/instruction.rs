use serde::{Deserialize, Serialize};

pub type InstructionValueType = i16;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Instruction {
    NoOp,
    TTL(InstructionValueType),
    ChangeFreeSpace(InstructionValueType),
    ChangeThermalCapacity(InstructionValueType),
    ChangeSystemHealth(InstructionValueType),
    ChangeOpenPorts(InstructionValueType),
}
