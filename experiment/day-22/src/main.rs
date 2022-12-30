use day_22::*;

fn main() {
    let input = include_str!("input.txt");
    let mut game: Game<150, 200> = build_game(input);
    let score = solve_part1(&mut game);
    println!("{:?}, {:?}", game.current_direction, game.current_position);
    println!("Part 1: {}", score);
}

pub fn build_game<const N: usize, const M: usize>(s: &str) -> Game<N, M> {
    let mut board_and_instructions = s.split("\n\n");
    let board_map = board_and_instructions.next().unwrap();
    let instructions = board_and_instructions.next().unwrap();
    let instructions = parse_instructions(instructions).unwrap().1;
    let board: Board<N, M> = parse_board(board_map);

    Game::new(board, instructions)
}

pub fn solve_part1<const N: usize, const M: usize>(game: &mut Game<N, M>) -> usize {
    let ((final_row, final_col), final_direction) = game.play();
    final_row * 1000 + final_col * 4 + final_direction.score()
}


#[cfg(test)]
mod tests {
    use day_22::*;
    use super::*;

    #[test]
    fn parse_works() {
        let input = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

        let mut game: Game<16, 12> = build_game(input);
        let score = solve_part1(&mut game);
        assert_eq!(score, 6032);
    }
}