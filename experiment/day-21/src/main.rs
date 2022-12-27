use day_21::{*, parse::statements};

pub fn get_input() -> Tree {
    let input = include_str!("input.txt");
    let stmts = statements(input).unwrap().1;
    stmts.into()
}

pub fn part1() {
    let tree = get_input();
    let values = tree.part1();
    println!("Part 1: {}", values.get("root").unwrap());
}


fn main() {
    part1();
}
