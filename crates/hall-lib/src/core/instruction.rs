use serde::{Deserialize, Serialize};

use crate::core::{AttributeKind, AttributeValueKind, Attributes, RuntimeAmountType};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum CardTargetValue {
    None,
    Machine,
    Actor,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum Amount {
    N(u8),
    Attribute(AttributeKind, AttributeValueKind),
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum RuntimeAmount {
    None,
    Value(Amount),
    Add(Amount, Amount),
    Sub(Amount, Amount),
    Mul(Amount, Amount),
    Div(Amount, Amount),
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum ValueTarget {
    None,
    FreeSpace,
    ThermalCapacity,
    SystemHealth,
    OpenPorts,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum LaunchInstruction {
    NoOp,
    Targ(CardTargetValue),
    Loop(RuntimeAmount),
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum RunInstruction {
    NoOp,
    IncV(ValueTarget, RuntimeAmount),
    DecV(ValueTarget, RuntimeAmount),
    CpCr,
    ReAu,
    ShIn(RuntimeAmount),
}

impl RuntimeAmount {
    fn resolve_value(amount: &Amount, attrs: &Attributes) -> RuntimeAmountType {
        match amount {
            Amount::N(val) => *val as RuntimeAmountType,
            Amount::Attribute(attr_kind, val_kind) => attrs.get_value(*attr_kind, *val_kind) as RuntimeAmountType,
        }
    }

    pub fn resolve(&self, attrs: &Attributes) -> RuntimeAmountType {
        match self {
            RuntimeAmount::None => 0,
            RuntimeAmount::Value(val) => Self::resolve_value(val, attrs),
            RuntimeAmount::Add(left, right) => Self::resolve_value(left, attrs).saturating_add(Self::resolve_value(right, attrs)),
            RuntimeAmount::Sub(left, right) => Self::resolve_value(left, attrs).saturating_sub(Self::resolve_value(right, attrs)),
            RuntimeAmount::Mul(left, right) => Self::resolve_value(left, attrs).saturating_mul(Self::resolve_value(right, attrs)),
            RuntimeAmount::Div(left, right) => Self::resolve_value(left, attrs).saturating_div(Self::resolve_value(right, attrs).max(1)),
        }
    }
}
