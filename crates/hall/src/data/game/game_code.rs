use serde::{Deserialize, Serialize};
use shared_data::game::opcode::OpCode;

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct GameCode(pub OpCode);

impl GameCode {
    pub(crate) fn execute(&self) -> bool {
        true
    }
}
