use std::collections::HashMap;
use nom::IResult;

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
    Literal(f64),
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

    pub fn evaluate_part1(&self, key: &str, memo: &mut HashMap<String, f64>) -> f64 {

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


    pub fn part1(&self) -> i64 {
        let mut memo = HashMap::new();
        self.evaluate_part1("root", &mut memo);
        memo.get("root").unwrap().round() as i64
    }

    /// Modify the tree slightly so that the root always subtracts its two subtrees and
    /// the `humn` is a given literal value.
    pub fn build_tree_for_part2(&self, x_coordinate: f64) -> Tree {
        let mut stmts = self.statements.clone();
        stmts
        .entry("root".to_string())
        .and_modify(
            |e| {
                match e {
                    Expression::Operation(op) => {
                        op.operator = Operator::Subtract;
                    },
                _ => {}
            }
        });
        stmts
        .entry("humn".to_string())
        .and_modify(
            |e| {
                match e {
                    Expression::Literal(_) => {
                    *e = Expression::Literal(x_coordinate);
                },
                _ => {}
            }
        });
        
        Tree {
            statements: stmts
        }
    }

    /// Idea:
    /// 
    /// We're assuming there's only one reference of `humn` in the tree so that
    /// arithmetic evaluation ends up being a linear equation in `humn`.
    /// A quick inspection of the dataset shows that this is the case.
    /// 
    /// Now we can try to find the zero of the function:
    /// f(humn) = value(left_child given `humn`) - value(right_child given `humn`)
    /// 
    /// Since this is a linear function, we can model it as y = mx + b for some gradient m 
    /// and intercept b. We can then solve for the value of `humn` that makes the function
    /// zero by rearranging the equation to:
    /// humn = -b / m
    /// 
    /// To calculate the gradient, we choose two points on the function and calculate the ratio of
    /// the difference in y values to the difference in x values. We have to be careful to choose large
    /// enough values for x so that the difference in y values is significant and the gradients rounds off to
    /// a nice whole number.
    pub fn part2(&self) -> i64 {

        // Just choose a wide enough step size to get a good enough answer.
        let x2 = 1000.0;
        let x1 = 0.0;

        let tree_at_x2 = self.build_tree_for_part2(x2);
        let tree_at_x1 = self.build_tree_for_part2(x1);

        let mut memo_x1 = HashMap::new();
        tree_at_x1.evaluate_part1("root", &mut memo_x1);
        let y1 = *memo_x1.get("root").unwrap();

        let mut memo_x2 = HashMap::new();
        tree_at_x2.evaluate_part1("root", &mut memo_x2);
        let y2 = *memo_x2.get("root").unwrap();

        let gradient = (y2 - y1) / (x2 - x1);
        let intercept = y1;

        (-intercept / gradient).round() as i64
    }

}


pub mod parse {
    use super::*;
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{one_of, alpha1, space1, i64 as parse_i64};
    use nom::combinator::map;
    use nom::multi::separated_list0;
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
            map(parse_i64, |v| Expression::Literal(v as f64))
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
            assert_eq!(expression("123"), Ok(("", Expression::Literal(123.0))));
        }

        #[test]
        fn test_statement() {
            assert_eq!(statement("a: 123"), Ok(("", Statement {
                target: "a".to_string(),
                value: Expression::Literal(123.0)
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

#[cfg(test)]
pub mod tests {
    use super::*;
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
        let tree: Tree = stmts.into();

        assert_eq!(tree.part1(), 152);
        assert_eq!(tree.part2(), 301);

    }
}