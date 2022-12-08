use rayon::prelude::*;


pub type Grid = Vec<Vec<u8>>;


pub fn is_visible(grid: &Grid, row_index: usize, column_index: usize) -> bool {
    let value = grid[row_index][column_index];
    (
        // Check the row remaining to the right.
        grid[row_index]
        .iter()
        .skip(column_index + 1)
        .all(|&item| item < value)
    ) ||
    (
        // Check the row remaining to the left.
        grid[row_index]
        .iter()
        .take(column_index)
        .all(|&item| item < value)
    ) ||
    (
        // Check the column above us.
        (0..grid.len())
        .take(row_index)
        .map(|row_index| grid[row_index][column_index])
        .all(|item| item < value)
    ) || 
    (
        // Check the column below us.
        (0..grid.len())
        .skip(row_index + 1)
        .map(|row_index| grid[row_index][column_index])
        .all(|item| item < value)
    )
}


pub fn get_scenic_score(grid: &Grid, row_index: usize, column_index: usize) -> usize {

    let value = grid[row_index][column_index];

    let mut first_same_height_seen: bool = false;

    let to_right = 
    grid[row_index]
        .iter()
        .skip(column_index + 1)
        .take_while(|&item| {
            if !first_same_height_seen {
                if *item >= value {
                    first_same_height_seen = true;
                }
                true
            } else {
                !first_same_height_seen && *item < value
            }
        })
        .count();

    if to_right == 0 { return 0 }

    let mut first_same_height_seen: bool = false;
    let to_left = 
    grid[row_index]
        .iter()
        .take(column_index)
        .rev()
        .take_while(|&item| {
            if !first_same_height_seen {
                if *item >= value {
                    first_same_height_seen = true;
                }
                true
            } else {
                !first_same_height_seen && *item < value
            }
        })
        .count();

    if to_left == 0 { return 0 }

    let mut first_same_height_seen: bool = false;
    let above = 
    (0..grid.len())
        .take(row_index)
        .map(|row_index| grid[row_index][column_index])
        .rev()
        .take_while(|&item| {
            if !first_same_height_seen {
                if item >= value {
                    first_same_height_seen = true;
                }
                true
            } else {
                !first_same_height_seen && item < value
            }
        })
        .count();

    if above == 0 { return 0 }

    let mut first_same_height_seen: bool = false;
    let below = 
    (0..grid.len())
        .skip(row_index + 1)
        .map(|row_index| grid[row_index][column_index])
        .take_while(|&item| {
            if !first_same_height_seen {
                if item >= value {
                    first_same_height_seen = true;
                }
                true
            } else {
                !first_same_height_seen && item < value
            }
        })
        .count();
    
    if below == 0 { return 0 }

    to_left * to_right * above * below
}


pub fn build_grid(input: &str) -> Grid {
    input
    .lines()
    .map(
        |line| 
            line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect()
    )
    .filter(|line: &Vec<u8>| line.len() != 0)
    .collect()
}

pub fn solve_part1(input: &str) -> usize {
    let grid = build_grid(input);
    
    // Loop over the cartesian product of row and col indices.
    (0..grid.len())
    .flat_map(|row_index| (0..grid[row_index].len())
    .map(move |col_index| (row_index, col_index)))

    // Collect into a Vec so that rayon can take it from here.
    .collect::<Vec<(usize, usize)>>()

    // Iterate in parallel and count all visible trees.
    .par_iter()
    .filter(|&&(row, col)| is_visible(&grid, row, col))
    .count()
}


pub fn solve_part2(input: &str) -> Option<usize> {
    let grid = build_grid(input);

    // Loop over the cartesian product of row and col indices.
    (0..grid.len())
    .flat_map(|row_index| (0..grid[row_index].len())
    .map(move |col_index| (row_index, col_index)))
    // Collect into a Vec to hand off to Rayon.
    .collect::<Vec<(usize, usize)>>()
    // Iterate over all index pairs in parallel to get the largest scenic score.
    .par_iter()
    .map(|&(row, col)| {
        get_scenic_score(&grid, row, col)
    })
    .max()
}


fn main() {
    let input = include_str!("input.txt");
    let part1 = solve_part1(input);
    let part2 = solve_part2(input).unwrap();

    println!("Part 1: {:#?}\nPart 2: {:#?}", part1, part2);
}


 #[cfg(test)]
 mod tests {
    use super::*;

    #[test]
    fn smol_grid() {
        let input: &str = "30373
25512
65332
33549
35390";
        let solution_part1 = solve_part1(input);
        assert_eq!(solution_part1, 21);

        let solution_part2 = solve_part2(input);
        assert_eq!(solution_part2, Some(8));
    }

    #[test]
    fn test_scenic_score() {
        let input: &str = "30373
25512
65332
33549
35390";
        let grid = build_grid(input);
        assert_eq!(get_scenic_score(&grid, 1, 2), 4);
        assert_eq!(get_scenic_score(&grid, 3, 2), 8);
        assert_eq!(get_scenic_score(&grid, 0, 2), 0);
    }
 }