use std::str::Chars;

use hall_lib::core::{Amount, AttributeKind, AttributeValueKind, CardTargetValue, LaunchInstruction, RunInstruction, RuntimeAmount, ValueTarget};

pub fn process_code_launch(instruction_str: &str) -> Option<LaunchInstruction> {
    use LaunchInstruction::*;

    let (code, remain) = instruction_str.split_once(':').unwrap_or((instruction_str, ""));

    match code {
        "TARG" => Some(Targ(parse_card_target(remain))),
        "LOOP" => Some(Loop(parse_runtime_expr(remain))),
        _ => None,
    }
}

pub fn process_code_run(instruction_str: &str) -> Option<RunInstruction> {
    use RunInstruction::*;

    let (code, remain) = instruction_str.split_once(':').unwrap_or((instruction_str, ""));

    match code {
        "INCV" => remain.split_once(':').map(|(target, value)| IncV(parse_value_target(target), parse_runtime_expr(value))),
        "DECV" => remain.split_once(':').map(|(target, value)| DecV(parse_value_target(target), parse_runtime_expr(value))),
        "CPCR" => Some(CpCr),
        "REAU" => Some(ReAu),
        "SHIN" => Some(ShIn(parse_runtime_expr(remain))),
        _ => None,
    }
}

fn parse_value_target(remain: &str) -> ValueTarget {
    match remain {
        "FS" => ValueTarget::FreeSpace,
        "OP" => ValueTarget::OpenPorts,
        "SH" => ValueTarget::SystemHealth,
        "TC" => ValueTarget::ThermalCapacity,
        _ => ValueTarget::None,
    }
}

fn parse_digits(tens: char, remain: &mut Chars) -> RuntimeAmount {
    let tens = tens.to_digit(10).unwrap_or(0);
    let value = remain.next().and_then(|c| c.to_digit(10)).map_or(tens, |ones| (tens * 10) + ones);
    RuntimeAmount::Value(Amount::N(value as u8))
}

fn parse_attr_value(attr_char: char, remain: &mut Chars) -> RuntimeAmount {
    let attr = match attr_char {
        'A' => AttributeKind::Analyze,
        'B' => AttributeKind::Breach,
        'C' => AttributeKind::Compute,
        'D' => AttributeKind::Disrupt,
        _ => return RuntimeAmount::None,
    };
    let value = match remain.next() {
        Some('A') => AttributeValueKind::Amplitude,
        Some('B') => AttributeValueKind::Beat,
        Some('C') => AttributeValueKind::Control,
        Some('D') => AttributeValueKind::Duration,
        _ => return RuntimeAmount::None,
    };

    RuntimeAmount::Value(Amount::Attribute(attr, value))
}

fn parse_runtime_amount(remain: &mut Chars) -> RuntimeAmount {
    match remain.next() {
        Some(attr @ 'A'..='D') => parse_attr_value(attr, remain),
        Some(tens @ '0'..='9') => parse_digits(tens, remain),
        _ => RuntimeAmount::None,
    }
}

fn parse_runtime_expr(remain: &str) -> RuntimeAmount {
    let mut chars = remain.chars();
    let amt_1 = parse_runtime_amount(&mut chars);
    let op = chars.next();
    let amt_2 = parse_runtime_amount(&mut chars);
    match (amt_1, op, amt_2) {
        (RuntimeAmount::Value(amt1), Some('+'), RuntimeAmount::Value(amt2)) => RuntimeAmount::Add(amt1, amt2),
        (RuntimeAmount::Value(amt1), Some('-'), RuntimeAmount::Value(amt2)) => RuntimeAmount::Sub(amt1, amt2),
        (RuntimeAmount::Value(amt1), Some('*'), RuntimeAmount::Value(amt2)) => RuntimeAmount::Mul(amt1, amt2),
        (RuntimeAmount::Value(amt1), Some('/'), RuntimeAmount::Value(amt2)) => RuntimeAmount::Div(amt1, amt2),
        (RuntimeAmount::Value(_), _, _) => amt_1,
        _ => RuntimeAmount::None,
    }
}

fn parse_card_target(remain: &str) -> CardTargetValue {
    match remain {
        "AA" => CardTargetValue::Actor,
        "MA" => CardTargetValue::Machine,
        _ => CardTargetValue::None,
    }
}

pub(crate) fn parse_rules_launch(rules: &str) -> Vec<LaunchInstruction> {
    rules.split('|').filter_map(process_code_launch).collect()
}
pub(crate) fn parse_rules_run(rules: &str) -> Vec<RunInstruction> {
    rules.split('|').filter_map(process_code_run).collect()
}

#[cfg(test)]
mod tests {
    use crate::data::rules::{process_code_launch, process_code_run};
    use hall_lib::core::{Amount::*, AttributeKind::*, AttributeValueKind::*, LaunchInstruction::*, RunInstruction::*, RuntimeAmount::*, ValueTarget::*};

    #[test]
    fn test_loop_one_digit() {
        let instruction = process_code_launch("LOOP:1");
        assert_eq!(instruction, Some(Loop(Value(N(1)))));
    }

    #[test]
    fn test_loop_two_digits() {
        let instruction = process_code_launch("LOOP:99");
        assert_eq!(instruction, Some(Loop(Value(N(99)))));
    }

    #[test]
    fn test_decv_two_digits() {
        let instruction = process_code_run("DECV:FS:05");
        assert_eq!(instruction, Some(DecV(FreeSpace, Value(N(5)))));
    }
    #[test]
    fn test_decv_attr() {
        let instruction = process_code_run("DECV:TC:AA");
        assert_eq!(instruction, Some(DecV(ThermalCapacity, Value(Attribute(Analyze, Amplitude)))));
    }
    #[test]
    fn test_decv_attr_add_value() {
        let instruction = process_code_run("DECV:SH:BB+10");
        assert_eq!(instruction, Some(DecV(SystemHealth, Add(Attribute(Breach, Beat), N(10)))));
    }
    #[test]
    fn test_decv_attr_mul_attr() {
        let instruction = process_code_run("DECV:OP:CC*DD");
        assert_eq!(instruction, Some(DecV(OpenPorts, Mul(Attribute(Compute, Control), Attribute(Disrupt, Duration)))));
    }
    #[test]
    fn test_incv_value_sub_attr() {
        let instruction = process_code_run("INCV:FS:10-AB");
        assert_eq!(instruction, Some(IncV(FreeSpace, Sub(N(10), Attribute(Analyze, Beat)))));
    }
    #[test]
    fn test_incv_value_add_value() {
        let instruction = process_code_run("INCV:TC:05+10");
        assert_eq!(instruction, Some(IncV(ThermalCapacity, Add(N(5), N(10)))));
    }
}
