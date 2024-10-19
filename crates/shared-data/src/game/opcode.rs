use serde::{Deserialize, Serialize};
use std::str::Chars;

type ContextValueType = i16;

#[derive(Clone, Serialize, Deserialize)]
pub enum OpCode {
    NoOp,
    ChangeFreeSpace(ContextValueType),
    ChangeThermalCapacity(ContextValueType),
    ChangeSystemHealth(ContextValueType),
    ChangeOpenPorts(ContextValueType),
}

impl OpCode {
    fn char_as_delta(c: char) -> ContextValueType {
        ((c as i32) - ('a' as i32)) as ContextValueType
    }

    pub fn process(c: char, chars: &mut Chars) -> Option<Self> {
        use OpCode::*;
        match c {
            'f' => {
                let amount_c = chars.next().unwrap_or_default();
                Some(ChangeFreeSpace(OpCode::char_as_delta(amount_c)))
            }
            't' => {
                let amount_c = chars.next().unwrap_or_default();
                Some(ChangeThermalCapacity(OpCode::char_as_delta(amount_c)))
            }
            's' => {
                let amount_c = chars.next().unwrap_or_default();
                Some(ChangeSystemHealth(OpCode::char_as_delta(amount_c)))
            }
            'o' => {
                let amount_c = chars.next().unwrap_or_default();
                Some(ChangeOpenPorts(OpCode::char_as_delta(amount_c)))
            }
            _ => None,
        }
    }
}
