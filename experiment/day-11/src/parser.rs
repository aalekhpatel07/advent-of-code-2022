use nom::{
    branch::alt,
    bytes::complete::*,
    character::{complete::*, streaming::one_of},
    combinator::{map, map_res, rest},
    error::context,
    multi::{many0, separated_list1},
    sequence::{pair, preceded, tuple},
    IResult,
};
use std::{
    ops::{Add, Div, Mul, Sub},
    str::FromStr,
};

use crate::Monke;
fn parse_monkey_index(s: &str) -> IResult<&str, usize> {
    map(
        context(
            "monkey index",
            pair(
                preceded(tag("Monkey "), map_res(digit1, str::parse::<usize>)),
                tag(":"),
            ),
        ),
        |(monke_index, _)| monke_index,
    )(s)
}

fn parse_starting_items(s: &str) -> IResult<&str, Vec<usize>> {
    preceded(
        tag("Starting items: "),
        separated_list1(tag(", "), map_res(digit1, str::parse::<usize>)),
    )(s)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operand {
    Unknown,
    Constant(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperation {
    Add,
    Multiply,
    Subtract,
    Divide,
}

impl BinaryOperation {
    pub fn operate<
        T: Add<T, Output = T> + Sub<T, Output = T> + Mul<T, Output = T> + Div<T, Output = T>,
    >(
        &self,
    ) -> impl Fn(T, T) -> T {
        match self {
            BinaryOperation::Add => Add::add,
            BinaryOperation::Multiply => Mul::mul,
            BinaryOperation::Subtract => Sub::sub,
            BinaryOperation::Divide => Div::div,
        }
    }
}

pub trait AsFn<T> {
    fn as_fn(&self) -> Box<dyn Fn(T) -> T>;
    fn apply(&self, item: T) -> T {
        self.as_fn()(item)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Operation {
    operand1: Operand,
    operand2: Operand,
    binary_op: BinaryOperation,
}

impl Default for Operation {
    fn default() -> Self {
        Self {
            operand1: Operand::Unknown,
            operand2: Operand::Unknown,
            binary_op: BinaryOperation::Add,
        }
    }
}

impl AsFn<usize> for Operation {
    fn as_fn(&self) -> Box<dyn Fn(usize) -> usize> {
        match (self.operand1, self.operand2, self.binary_op) {
            (Operand::Unknown, Operand::Unknown, bin_op) => {
                Box::new(move |item| bin_op.operate()(item, item))
            }
            (Operand::Unknown, Operand::Constant(constant), bin_op) => {
                Box::new(move |item| bin_op.operate()(item, constant))
            }
            (Operand::Constant(constant), Operand::Unknown, bin_op) => {
                Box::new(move |item| bin_op.operate()(item, constant))
            }
            _ => unimplemented!("Must have at least one unknown (i.e. \"old\")."),
        }
    }
}

fn parse_operand(s: &str) -> IResult<&str, Operand> {
    alt((
        map(tag("old"), |_| Operand::Unknown),
        map(digit1, |digit_str: &str| {
            Operand::Constant(digit_str.parse::<usize>().unwrap())
        }),
    ))(s)
}

fn parse_binary_op(s: &str) -> IResult<&str, BinaryOperation> {
    let (i, t) = one_of("+-*/")(s)?;
    Ok((
        i,
        match t {
            '+' => BinaryOperation::Add,
            '-' => BinaryOperation::Subtract,
            '*' => BinaryOperation::Multiply,
            '/' => BinaryOperation::Divide,
            _ => unimplemented!("Only +-* implemented."),
        },
    ))
}

fn parse_operation(s: &str) -> IResult<&str, Operation> {
    let (rem, (op1, _, bin_op, _, op2)) = tuple((
        parse_operand,
        space1,
        parse_binary_op,
        space1,
        parse_operand,
    ))(s)?;
    Ok((
        rem,
        Operation {
            operand1: op1,
            operand2: op2,
            binary_op: bin_op,
        },
    ))
}

fn parse_operation_str(s: &str) -> IResult<&str, Operation> {
    let (_, expr_str) = preceded(tag("Operation: new = "), rest)(s)?;

    let (rem, op) = parse_operation(expr_str)?;
    Ok((rem, op))
}

fn parse_prefix_then_number<'a, 'b, T: FromStr>(
    s: &'a str,
    prefix: &'b str,
) -> IResult<&'a str, T> {
    preceded(
        tag(prefix),
        map_res(digit1, |digit_str: &str| digit_str.parse::<T>()),
    )(s)
}

fn parse_divisibility_test(s: &str) -> IResult<&str, usize> {
    preceded(
        tag("Test: divisible by "),
        map_res(digit1, |digit_str: &str| digit_str.parse::<usize>()),
    )(s)
}

pub fn parse_monke(s: &str) -> IResult<&str, Monke> {
    let (mut s, monke_index) = parse_monkey_index(s)?;
    s = s.trim_start();
    let (mut s, starting_items) = parse_starting_items(s)?;
    s = s.trim_start();
    let (mut s, operation) = parse_operation_str(s)?;
    s = s.trim_start();
    let (mut s, divisor) = parse_divisibility_test(s)?;
    s = s.trim_start();
    let (mut s, true_case_target_monkey) =
        parse_prefix_then_number::<usize>(s, "If true: throw to monkey ")?;
    s = s.trim_start();
    let (mut s, false_case_target_monkey) =
        parse_prefix_then_number::<usize>(s, "If false: throw to monkey ")?;
    s = s.trim_start();

    Ok((
        s,
        Monke {
            index: monke_index,
            items: starting_items,
            operation,
            test_divisor: divisor,
            throw_to_monkey_if_test: true_case_target_monkey,
            throw_to_monkey_if_not_test: false_case_target_monkey,
        },
    ))
}

pub fn parse_many_monke(s: &str) -> IResult<&str, Vec<Monke>> {
    many0(parse_monke)(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_operand() {
        let (_, operand) = parse_operand("old").unwrap();
        assert_eq!(operand, Operand::Unknown);
        let (_, operand) = parse_operand("1234").unwrap();
        assert_eq!(operand, Operand::Constant(1234));
    }
    #[test]
    fn test_parse_binary_op() {
        let (_, val) = parse_binary_op("+").unwrap();
        assert_eq!(val, BinaryOperation::Add);
        let (_, val) = parse_binary_op("-").unwrap();
        assert_eq!(val, BinaryOperation::Subtract);
        let (_, val) = parse_binary_op("*").unwrap();
        assert_eq!(val, BinaryOperation::Multiply);
        let (_, val) = parse_binary_op("/").unwrap();
        assert_eq!(val, BinaryOperation::Divide);
    }

    #[test]
    fn test_parse_operation() {
        let (_, val) = parse_operation("old * 19").unwrap();
        assert_eq!(val.binary_op, BinaryOperation::Multiply);
        assert_eq!(val.operand2, Operand::Constant(19));
        assert_eq!(val.operand1, Operand::Unknown);
    }

    #[test]
    fn test_operation_apply() {
        let (_, val) = parse_operation("old * 19").unwrap();
        assert_eq!(val.binary_op, BinaryOperation::Multiply);
        assert_eq!(val.operand2, Operand::Constant(19));
        assert_eq!(val.operand1, Operand::Unknown);
        assert_eq!(val.apply(3), 57);

        let (_, val) = parse_operation("old    + old").unwrap();
        assert_eq!(val.binary_op, BinaryOperation::Add);
        assert_eq!(val.operand2, Operand::Unknown);
        assert_eq!(val.operand1, Operand::Unknown);
        assert_eq!(val.apply(3), 6);
    }

    #[test]
    fn test_parse_operation_str() {
        let (_, operation) = parse_operation_str("Operation: new = old * 19").unwrap();
        assert_eq!(operation.binary_op, BinaryOperation::Multiply);
        assert_eq!(operation.operand1, Operand::Unknown);
        assert_eq!(operation.operand2, Operand::Constant(19));
        assert_eq!(operation.apply(3), 57);
    }
    #[test]
    fn test_parse_divisibility_test_str() {
        let (_, div_test) = parse_divisibility_test("Test: divisible by 19").unwrap();
        assert_eq!(div_test, 19);
    }

    #[test]
    fn test_parse_monke() -> Result<(), Box<dyn std::error::Error>> {
        let monke = "Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
  If true: throw to monkey 2
  If false: throw to monkey 3";
        let (_, parsed_monke) = parse_monke(monke)?;

        assert_eq!(parsed_monke.index, 0);
        assert_eq!(&parsed_monke.items, &vec![79, 98]);
        assert_eq!(parsed_monke.operation.apply(3), 57);
        assert_eq!(parsed_monke.test_divisor, 23);
        assert_eq!(parsed_monke.throw_to_monkey_if_not_test, 3);
        assert_eq!(parsed_monke.throw_to_monkey_if_test, 2);
        Ok(())
    }

    #[test]
    fn test_parse_many_monke() -> Result<(), Box<dyn std::error::Error>> {
        let s = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
        let (_, parsed_monkes) = parse_many_monke(s)?;
        assert_eq!(parsed_monkes.len(), 4);
        Ok(())
    }
}
