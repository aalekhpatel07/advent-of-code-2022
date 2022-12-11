use day_11::*;
use std::collections::HashMap;

fn main() {
    let contents = include_str!("input.txt");
    let res = solve_part1(contents);
    println!("Part 1: {res}");
    let res = solve_part2(contents);
    println!("Part 2: {res}");
}

pub fn solve(contents: &str, rounds: usize, is_part_two: bool) -> usize {
    let (_, monkees) = parse_many_monke(contents).unwrap();

    let mut hmap: HashMap<MonkeIdx, usize> = HashMap::new();
    for idx in 0..monkees.len() {
        hmap.insert(idx, 0);
    }

    let divisor_product = monkees.iter().map(|monke| monke.test_divisor).product();

    let mut game = MonkeBusiness {
        monkees,
        inspection_count: hmap,
        is_part_two,
        divisor_product,
    };

    for _ in 0..rounds {
        game.play_round();
    }

    let mut counts = game.inspection_count.into_values().collect::<Vec<usize>>();
    counts.sort();
    counts.reverse();
    counts[0] * counts[1]
}

pub fn solve_part1(contents: &str) -> usize {
    solve(contents, 20, false)
}
pub fn solve_part2(contents: &str) -> usize {
    solve(contents, 10_000, true)
}

#[cfg(test)]
mod tests {
    use crate::{solve_part1, solve_part2};

    #[test]
    fn test_smol_game() {
        let contents: &str = "Monkey 0:
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

        let res = solve_part1(contents);
        assert_eq!(res, 101 * 105);

        let res = solve_part2(contents);
        assert_eq!(res, 52166 * 52013);
    }
}
