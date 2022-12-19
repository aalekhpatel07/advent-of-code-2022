use day_17::*;

const INPUT: &str = include_str!("input.txt");


fn main() {
    let directions = get_directions(INPUT);
    println!("Part 1: {}", solve_part1(directions.clone(), 2022));
    println!("Part 2: {}", solve_part1(directions, 1000000000000));
}

pub fn get_directions(s: &str) -> Directions {
    parse_direction(s).unwrap().1.into()
}


pub fn solve_part1(
    directions: Directions, 
    num_rocks: usize
) -> usize {

    let mut cave = Cave::new(directions, 7);
    cave.quiet = true;
    cave.log_spawn = false;
    cave.run(num_rocks);

    cave.height()
}

pub fn solve_part2(directions: Directions) {

}

#[cfg(test)]
mod tests {
    use super::*;
    use day_17::*;

    #[test]
    fn test_smol_part1() {
        let s = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        let directions = get_directions(s);
        let result = solve_part1(directions, 2022);
        assert_eq!(result, 3068);
    }

}
