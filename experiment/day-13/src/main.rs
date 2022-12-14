use std::cmp::Ordering;

use day_13::*;


pub fn solve_part1(s: &str) -> usize {

    s
    .split("\n\n")
    .into_iter()
    .enumerate()
    .map(|(idx, pair)| {
        let item = pair.split("\n").collect::<Vec<&str>>();
        let first = Packet::parse(item[0]).unwrap().1;
        let second = Packet::parse(item[1]).unwrap().1;

        if first.partial_cmp(&second) == Some(Ordering::Less) {
            idx + 1
        } else {
            0
        }
    })
    .sum()

}

pub fn solve_part2(s: &str) -> usize {

    let mut pairs = vec![];

    s
    .split("\n\n")
    .into_iter()
    .for_each(|pair| {
        let item = pair.split("\n").collect::<Vec<&str>>();
        let first = Packet::parse(item[0]).unwrap().1;
        let second = Packet::parse(item[1]).unwrap().1;

        pairs.push(first);
        pairs.push(second);
    });

    let divider_packet1 = Packet::parse("[[2]]").unwrap().1;
    let divider_packet2 = Packet::parse("[[6]]").unwrap().1;

    pairs.push(divider_packet1.clone());
    pairs.push(divider_packet2.clone());

    pairs.sort();

    let divider_packet1_index = 
        pairs
        .iter()
        .enumerate()
        .filter(
            |(_, packet)| {
                packet == &&divider_packet1
            }
        )
        .map(|(idx, _)| idx)
        .next()
        .unwrap();


    let divider_packet2_index = 
        pairs
        .iter()
        .enumerate()
        .filter(
            |(_, packet)| {
                packet == &&divider_packet2
            }
        )
        .map(|(idx, _)| idx)
        .next()
        .unwrap();

    (divider_packet1_index + 1) * (divider_packet2_index + 1)
}


fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", solve_part1(input));
    println!("Part 2: {}", solve_part2(input));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let s = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

        assert_eq!(solve_part1(s), 13);
        assert_eq!(solve_part2(s), 140);
    }
}