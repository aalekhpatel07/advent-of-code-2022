use day_21::{*, parse::statements};

pub fn main() {
    let input = include_str!("input.txt");
    let stmts = statements(input).unwrap().1;
    let tree: Tree = stmts.into();
    
    println!("Part 1: {}", tree.part1());
    println!("Part 2: {}", tree.part2());
}