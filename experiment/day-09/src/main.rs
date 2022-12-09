use std::ops::AddAssign;
use std::sync::{Arc, Mutex};
use std::{collections::HashSet, str::FromStr};
use std::result::Result;
use crossbeam::channel::{
    Sender,
    Receiver,
    unbounded
};


#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct Position {
    pub row: isize,
    pub col: isize
}

impl From<(isize, isize)> for Position {
    fn from((row, col): (isize, isize)) -> Self {
        Self {row, col}
    }
}


impl Position {

    // Get the direction of jump that `self` must make 
    // to stay close to other.
    pub fn two_steps_in_direction(&self, other: &Position) -> Option<Direction> {
        
        if self.distance(other) < 2 {
            return None
        }

        // On the same column. Can go either up or down.
        if self.column_distance(other) == 0 {
            if other.row > self.row {
                Some(Direction::Up)
            } else {
                Some(Direction::Down)
            }
        } 
        // On the same row. Can go either left or right.
        else if self.row_distance(other) == 0 {
            if other.col > self.col {
                Some(Direction::Right)
            } else {
                Some(Direction::Left)
            }
        } else {
            // Neither on same column nor on same height,
            // so make a diagonal jump.
            match (other.col > self.col, other.row > self.row) {
                (true, true) => Some(Direction::UpRight),
                (true, false) => Some(Direction::DownRight),
                (false, true) => Some(Direction::UpLeft),
                (false, false) => Some(Direction::DownLeft)
            }
        }


    }
    pub fn row_distance(&self, other: &Position) -> isize {
        (self.row - other.row).abs()
    }
    pub fn column_distance(&self, other: &Position) -> isize {
        (self.col - other.col).abs()
    }

    pub fn distance(&self, other: &Position) -> isize {
        self.row_distance(other).max(self.column_distance(other))
    }
}


impl Default for Position {
    fn default() -> Self {
        Self {
            row: 0, col: 0
        }
    }
}


impl AddAssign<Position> for Position {
    fn add_assign(&mut self, rhs: Position) {
        self.row += rhs.row;
        self.col += rhs.col;
    } 
}


#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Right,
    Up,
    Left,
    Down,
    UpRight,
    DownRight,
    UpLeft,
    DownLeft
}

impl Direction {
    pub fn vector(&self) -> Position {
        match self {
            Self::Down => (-1, 0).into(),
            Self::Left => (0, -1).into(),
            Self::Right => (0, 1).into(),
            Self::Up => (1, 0).into(),
            Self::UpLeft => (1, -1).into(),
            Self::UpRight => (1, 1).into(),
            Self::DownLeft => (-1, -1).into(),
            Self::DownRight => (-1, 1).into(),
        }
    }
}


impl FromStr for Direction {
    type Err = std::io::Error;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "R" => Ok(Direction::Right),
            "L" => Ok(Direction::Left),
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            _ => panic!("Unknown char.")
        }
    }
}

#[derive(Debug, Clone)]
pub struct Command {
    pub direction: Direction,
    pub steps: usize
}

impl FromStr for Command {
    type Err = std::io::Error;
    fn from_str(value: &str) -> Result<Self, Self::Err> {

        let direction: String = value.chars().take(1).collect::<String>();
        let count: String = value.chars().skip(2).collect::<String>();
        let count = usize::from_str(&count).unwrap();

        Ok(Self {
            direction: Direction::from_str(&direction)?,
            steps: count
        })
    }
}

#[derive(Debug, Clone)]
pub struct Game<const N: usize> {
    pub rx: Receiver<Command>,
    pub knots: Vec<Position>,
    pub coverage: Vec<HashSet<Position>>
}


impl<const N: usize> Game<N> {

    pub fn new(rx: Receiver<Command>) -> Self {
        Self {
            rx,
            knots: vec![Position::default(); N],
            coverage: vec![HashSet::from_iter([Position::default()]); N]
        }
    }

    pub fn accept(&mut self, command: Command) {
        let Command { direction, steps } = command;

        // Update the positions iteratively.
        for _ in 1..=steps {
            // First the leader knot gets to update.
            let mut leader_knot = self.knots[0];

            leader_knot += direction.vector();
            self.knots[0] = leader_knot;
            self.coverage.get_mut(0).unwrap().insert(leader_knot);
            
            // Once the leader knot moves we update all following knots
            // in order, depending on the most recent position of the knot
            // preceding it.

            for index in 1..self.knots.len() {
                let mut follower_knot = self.knots[index];

                if let Some(direction_for_follower) = follower_knot.two_steps_in_direction(&self.knots[index - 1]) {
                    follower_knot += direction_for_follower.vector();
                    self.coverage.get_mut(index).unwrap().insert(follower_knot);
                    self.knots[index] = follower_knot;
                }
            }
        }
    }

    pub fn run(&mut self) {
        while let Ok(command) = self.rx.recv() {
            self.accept(command);
        }
    }
}


pub fn solve_part1(input: &str) -> usize {
    let (tx, rx) = unbounded();
    let game = Game::<2>::new(rx);
    solve(input, game, 1, tx)
}

pub fn solve_part2(input: &str) -> usize {
    let (tx, rx) = unbounded();
    let game = Game::<10>::new(rx);
    solve(input, game, 9, tx)
}

/// Solve for a general scenario of the knots game.
pub fn solve<const N: usize>(input: &str, game: Game<N>, knot_to_track: usize, tx: Sender<Command>) -> usize {
    let game = Arc::new(Mutex::new(game));

    let game_cp = game.clone();
    let handle = std::thread::spawn(move || {
        game_cp.lock().unwrap().run();
    });

    let commands = 
        input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| Command::from_str(line).unwrap());

    for command in commands {
        tx.send(command).unwrap();
    }
    
    drop(tx);
    handle.join().unwrap();

    let size_of_different_positions = game.lock().unwrap().coverage.get(knot_to_track).unwrap().len();
    size_of_different_positions
}



pub fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", solve_part1(input));
    println!("Part 1: {}", solve_part2(input));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smol_case() {
        let input: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        assert_eq!(13, solve_part1(input));
        assert_eq!(1, solve_part2(input));
    }

    #[test]
    fn test_big_case() {
        let input: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        // assert_eq!(13, solve_part1(input));
        assert_eq!(36, solve_part2(input));
    }


}