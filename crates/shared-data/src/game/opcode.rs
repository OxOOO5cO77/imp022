use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum OpCode {
    NoOp,
}

impl From<char> for OpCode {
    fn from(c: char) -> Self {
        use OpCode::*;
        NoOp
    }
}
