use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BinaryHeap;
use pathfinding::prelude::astar;


pub struct Grid(Vec<Vec<char>>);


pub fn build_grid(s: &str) -> Grid {
    Grid(
        s
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
    )
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Step {
    pub cost: isize,
    pub index: (usize, usize)
}




impl Grid {
    pub fn get_neighbors(&self, row_index: usize, col_index: usize) -> impl Iterator<Item=(usize, usize)> {
        let mut current = *self.0.get(row_index).unwrap().get(col_index).unwrap() as u8;
        if current == 'S' as u8 {
            current = 'a' as u8;
        }
        let mut candidates = vec![];

        if row_index > 0 {

            let row = self.0.get(row_index - 1).unwrap();
            let mut entry = *row.get(col_index).unwrap() as u8;

            if entry == 'E' as u8 {
                entry = 'z' as u8;
            }
            if entry <= current + 1 {
                candidates.push((row_index - 1, col_index));
            }
        }

        if row_index < self.0.len() - 1 {

            let row = self.0.get(row_index + 1).unwrap();
            let mut entry = *row.get(col_index).unwrap() as u8;

            if entry == 'E' as u8 {
                entry = 'z' as u8;
            }
            if entry <= current + 1 {
                candidates.push((row_index + 1, col_index));
            }
        }


        if col_index > 0 {

            let row = self.0.get(row_index).unwrap();
            let mut entry = *row.get(col_index - 1).unwrap() as u8;

            if entry == 'E' as u8 {
                entry = 'z' as u8;
            }
            if entry <= current + 1 {
                candidates.push((row_index, col_index - 1));
            }
        }


        if col_index < self.0.get(row_index).unwrap().len() - 1 {

            let row = self.0.get(row_index).unwrap();
            let mut entry = *row.get(col_index + 1).unwrap() as u8;

            if entry == 'E' as u8 {
                entry = 'z' as u8;
            }
            if entry <= current + 1 {
                candidates.push((row_index, col_index + 1));
            }
        }

        candidates.into_iter()
    }

    pub fn find(&self, c: char) -> Option<(usize, usize)> {
        (0..self.0.len())
        .flat_map(
            |row_index| (0..self.0.get(row_index).unwrap().len()).clone().map(move |col_index| (row_index, col_index))
        )
        .filter(|&(row_index, col_index)| *self.0.get(row_index).unwrap().get(col_index).unwrap() == c)
        .next()
    }

    pub fn find_all(&self, c: char) -> Vec<(usize, usize)> {
        (0..self.0.len())
        .flat_map(
            |row_index| (0..self.0.get(row_index).unwrap().len()).clone().map(move |col_index| (row_index, col_index))
        )
        .filter(|&(row_index, col_index)| *self.0.get(row_index).unwrap().get(col_index).unwrap() == c)
        .collect()
    }


    // My initial attempt but this is too slow on grids.
    // A* is wayy better.
    pub fn shortest_path_dijkstra(&self) -> isize {
        let start_index = self.find('S').unwrap();
        let mut distances: HashMap<(usize, usize), isize> = HashMap::new();
        let end_index = self.find('E').unwrap();

        let mut prev: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
        let mut visited = HashSet::<(usize, usize)>::new();
        let mut priority_queue: BinaryHeap<Step> = BinaryHeap::new();

        priority_queue.push(Step { cost: 0, index: start_index });
        distances.entry(start_index).or_insert(0);

        while let Some(Step { cost: inverted_cost, index }) = priority_queue.pop() {
            let cost = -inverted_cost;
            visited.insert(index);

            if index == end_index {
                distances.entry(end_index).or_insert(cost);
                prev.insert(end_index, index);
                break;
            }

            for neighbor in self.get_neighbors(index.0, index.1) {
                if visited.contains(&neighbor) {
                    continue;
                }

                let dist = distances.entry(neighbor).or_insert(isize::MAX);

                if *dist >= cost + 1 {
                    *dist = cost + 1;
                    prev.insert(neighbor, index);
                }
                priority_queue.push(Step { cost: -(cost + 1), index: neighbor });
            }
        }

        distances[&end_index]
    }

    
    pub fn shortest_path_a_star(
        &self, 
        start_index: (usize, usize), 
        end_index: (usize, usize),
    ) -> usize {

        if let Some((_path, cost)) = astar(
            &start_index,
            |p| self.get_neighbors(p.0, p.1).map(|p| (p, 1)),
            |p| (p.0.abs_diff(end_index.0) + p.1.abs_diff(end_index.1)) as usize,
            |p| *p == end_index
        ) {
            cost
        } else {
            usize::MAX
        }
    }

}


pub fn solve_part1(s: &str) -> String {
    let grid = build_grid(s);
    let start_index = grid.find('S').unwrap();
    let end_index = grid.find('E').unwrap();
    let cost = grid.shortest_path_a_star(start_index, end_index);
    (cost).to_string()
}

pub fn solve_part2(s: &str) -> String {
    let grid = build_grid(s);

    let end_index = grid.find('E').unwrap();

    let mut start_indices = grid.find_all('a');
    start_indices.push(grid.find('S').unwrap());

    start_indices
    .iter()
    .map(|&start_index| {
        grid.shortest_path_a_star(start_index, end_index)
    })
    .min()
    .unwrap()
    .to_string()
}


fn main() {
    println!("Part 1: {}", solve_part1(include_str!("input.txt")));
    println!("Part 2: {}", solve_part2(include_str!("input.txt")));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_grid() {
        let grid = build_grid("abc\ndef\nghi");
        assert_eq!(grid.0, vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f'], vec!['g', 'h', 'i']]);
    }

    #[test]
    fn test_shortest_path() {
        let raw = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";


        let grid = build_grid(raw);
        let start_index = grid.find('S').unwrap();
        let end_index = grid.find('E').unwrap();
        let cost = grid.shortest_path_a_star(start_index, end_index);

        assert_eq!(cost, 31);
        // println!("{:#?}", path);

    }

    #[test]
    fn test_build_grid_big_start_and_end_present() {
        let grid = build_grid(include_str!("input.txt"));
        assert!(grid.0.iter().any(|row| row.iter().any(|&c| c == 'S')));
        assert!(grid.0.iter().any(|row| row.iter().any(|&c| c == 'E')));

        // assert_eq!(grid, vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f'], vec!['g', 'h', 'i']]);
    }


    #[test]
    fn test_get_neighbors() {
        let raw = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        let grid = build_grid(raw);
        let mut neighbors = grid.get_neighbors(1, 1).collect::<Vec<(usize, usize)>>();
        neighbors.sort();
        assert_eq!(neighbors, vec![(0, 1), (1, 0), (1, 2), (2, 1)]);
    }
}