use std::str::Chars;

use hall::data::core::{Amount, AttributeKind, AttributeValueKind, Instruction, RuntimeAmount, ValueTarget};

pub fn process_code(instruction_str: &str) -> Option<Instruction> {
    use Instruction::*;

    let (code, remain) = instruction_str.split_once(':').unwrap_or((instruction_str, ""));

    match code {
        "TTL" => Some(TTL(parse_runtime_expr(remain))),
        "INC" => remain.split_once(':').map(|(target, value)| INC(parse_value_target(target), parse_runtime_expr(value))),
        "DEC" => remain.split_once(':').map(|(target, value)| DEC(parse_value_target(target), parse_runtime_expr(value))),
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
        Some('A') => AttributeValueKind::Accuracy,
        Some('B') => AttributeValueKind::Boost,
        Some('C') => AttributeValueKind::Celerity,
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

pub(crate) fn parse_rules(rules: &str) -> Vec<Instruction> {
    rules.split('|').filter_map(process_code).collect()
}

#[cfg(test)]
mod tests {
    use crate::data::rules::process_code;
    use hall::data::core::{Amount::*, AttributeKind::*, AttributeValueKind::*, Instruction::*, RuntimeAmount::*, ValueTarget::*};

    #[test]
    fn test_ttl_one_digit() {
        let instruction = process_code("TTL:1");
        assert_eq!(instruction, Some(TTL(Value(N(1)))));
    }

    #[test]
    fn test_ttl_two_digits() {
        let instruction = process_code("TTL:99");
        assert_eq!(instruction, Some(TTL(Value(N(99)))));
    }

    #[test]
    fn test_dec_two_digits() {
        let instruction = process_code("DEC:FS:05");
        assert_eq!(instruction, Some(DEC(FreeSpace, Value(N(5)))));
    }
    #[test]
    fn test_dec_attr() {
        let instruction = process_code("DEC:TC:AA");
        assert_eq!(instruction, Some(DEC(ThermalCapacity, Value(Attribute(Analyze, Accuracy)))));
    }
    #[test]
    fn test_dec_attr_add_value() {
        let instruction = process_code("DEC:SH:BB+10");
        assert_eq!(instruction, Some(DEC(SystemHealth, Add(Attribute(Breach, Boost), N(10)))));
    }
    #[test]
    fn test_dec_attr_mul_attr() {
        let instruction = process_code("DEC:OP:CC*DD");
        assert_eq!(instruction, Some(DEC(OpenPorts, Mul(Attribute(Compute, Celerity), Attribute(Disrupt, Duration)))));
    }
    #[test]
    fn test_inc_value_sub_attr() {
        let instruction = process_code("INC:FS:10-AB");
        assert_eq!(instruction, Some(INC(FreeSpace, Sub(N(10), Attribute(Analyze, Boost)))));
    }
    #[test]
    fn test_inc_value_add_value() {
        let instruction = process_code("INC:TC:05+10");
        assert_eq!(instruction, Some(INC(ThermalCapacity, Add(N(5), N(10)))));
    }
}
