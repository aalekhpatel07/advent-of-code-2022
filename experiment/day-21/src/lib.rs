use std::collections::HashMap;

use nom::{
    IResult
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide
}


#[derive(Debug, Clone, PartialEq)]
pub struct Operation {
    pub operator: Operator,
    pub left: String,
    pub right: String
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(u64),
    Operation(Operation)
}


#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    pub target: String,
    pub value: Expression
}


#[derive(Debug, Clone)]
pub struct Tree {
    pub statements: HashMap<String, Expression>,
}

impl From<Vec<Statement>> for Tree {
    fn from(statements: Vec<Statement>) -> Self {
        let mut tree = Tree::new();
        for statement in statements {
            tree.add_statement(statement);
        }
        tree
    }
}


impl Tree {
    pub fn new() -> Self {
        Self {
            statements: HashMap::new(),
        }
    }

    pub fn add_statement(&mut self, statement: Statement) {
        self.statements.insert(statement.target.clone(), statement.value);
    }

    pub fn evaluate_part1(&self, key: &str, memo: &mut HashMap<String, u64>) -> u64 {

        if let Some(value) = memo.get(key) {
            return *value;
        } else {

            let Some(statement) = self.statements.get(key) else {
                panic!("Unknown variable {}", key);
            };

            match statement {
                Expression::Literal(v) => {
                    // self.value_map.insert(key.to_string(), *v);
                    *v
                },
                Expression::Operation(op) => {
                    let left = self.evaluate_part1(&op.left, memo);
                    let right = self.evaluate_part1(&op.right, memo);

                    let result = match op.operator {
                        Operator::Add => std::ops::Add::add(left, right),
                        Operator::Subtract => std::ops::Sub::sub(left, right),
                        Operator::Multiply => std::ops::Mul::mul(left, right),
                        Operator::Divide => std::ops::Div::div(left, right),
                    };

                    memo.insert(key.to_string(), result);
                    result
                }
            }
        }
    }


    pub fn part1(&self) -> HashMap<String, u64> {
        let mut memo = HashMap::new();
        self.evaluate_part1("root", &mut memo);
        memo
    }

}


pub mod parse {
    use super::*;
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{one_of, alpha1, space1, u64 as parse_u64};
    use nom::combinator::map;
    use nom::multi::{many1, separated_list1, separated_list0};
    use nom::sequence::tuple;


    pub fn operator(s: &str) -> IResult<&str, Operator> {
        map(
            one_of("+-/*"),
            |c| match c {
                '+' => Operator::Add,
                '-' => Operator::Subtract,
                '*' => Operator::Multiply,
                '/' => Operator::Divide,
                _ => unreachable!("Expected one of +-/*")
            }
        )(s)
    }

    pub fn operation(s: &str) -> IResult<&str, Operation> {
        map(
            tuple((alpha1, space1, operator, space1, alpha1)),
            |(left, _, operator, _, right)|Operation {
                operator,
                left: left.to_string(),
                right: right.to_string()
            }
        )(s)
    }

    pub fn expression(s: &str) -> IResult<&str, Expression> {
        alt((
            map(operation, |op| Expression::Operation(op)),
            map(parse_u64, |v| Expression::Literal(v))
        ))(s)
    }

    pub fn statement(s: &str) -> IResult<&str, Statement> {
        map(
            tuple((alpha1, tag(":"), space1, expression)),
            |(target, _, _, value)| Statement {
                target: target.to_string(),
                value
            }
        )(s)
    }

    pub fn statements(s: &str) -> IResult<&str, Vec<Statement>> {
        separated_list0(tag("\n"), statement)(s)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::*;

        #[test]
        fn test_operator() {
            assert_eq!(operator("+"), Ok(("", Operator::Add)));
            assert_eq!(operator("-"), Ok(("", Operator::Subtract)));
            assert_eq!(operator("*"), Ok(("", Operator::Multiply)));
            assert_eq!(operator("/"), Ok(("", Operator::Divide)));
        }

        #[test]
        fn test_operation() {
            assert_eq!(operation("a + b"), Ok(("", Operation {
                operator: Operator::Add,
                left: "a".to_string(),
                right: "b".to_string()
            })));
            assert_eq!(operation("a - b"), Ok(("", Operation {
                operator: Operator::Subtract,
                left: "a".to_string(),
                right: "b".to_string()
            })));
            assert_eq!(operation("a * b"), Ok(("", Operation {
                operator: Operator::Multiply,
                left: "a".to_string(),
                right: "b".to_string()
            })));
            assert_eq!(operation("a / b"), Ok(("", Operation {
                operator: Operator::Divide,
                left: "a".to_string(),
                right: "b".to_string()
            })));
        }

        #[test]
        fn test_expression() {
            assert_eq!(expression("a + b"), Ok(("", Expression::Operation(Operation {
                operator: Operator::Add,
                left: "a".to_string(),
                right: "b".to_string()
            }))));
            assert_eq!(expression("123"), Ok(("", Expression::Literal(123))));
        }

        #[test]
        fn test_statement() {
            assert_eq!(statement("a: 123"), Ok(("", Statement {
                target: "a".to_string(),
                value: Expression::Literal(123)
            })));
            assert_eq!(statement("ccza: ac + qb"), Ok(("", Statement {
                target: "ccza".to_string(),
                value: Expression::Operation(Operation {
                    operator: Operator::Add,
                    left: "ac".to_string(),
                    right: "qb".to_string()
                })
            })));
        }

    }
}


// pub fn parse_statement(s: &str) -> IResult<Statement, &str> {

// }

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::*;
    use super::parse::statements;

    #[test]
    pub fn test_tree() {
        let input = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

        let stmts = statements(input).unwrap().1;
        println!("{:#?}", stmts);
        let tree: Tree = stmts.into();
        let memo = tree.part1();
        println!("{:#?}", memo);


        // let tree = Tree::from_statements(stmts);

    }
}