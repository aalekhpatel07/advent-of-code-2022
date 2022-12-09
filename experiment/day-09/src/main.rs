use std::ops::{Add, AddAssign};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::{collections::HashSet, str::FromStr, char::ParseCharError};
use std::result::Result;
use crossbeam::channel::{
    Sender,
    Receiver,
    unbounded
};
use anyhow::Error;
use anyhow::anyhow;


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
    pub fn close_enough(&self, other: &Position) -> bool {
        self.distance(other) <= 1
    }
    pub fn two_steps_in_direction(&self, other: &Position) -> Option<Direction> {
        
        if self.distance(other) < 2 {
            return None
        }

        if self.column_distance(other) == 0 {
            if other.row > self.row {
                return Some(Direction::Up)
            }
            return Some(Direction::Down)

        } else if self.row_distance(other) == 0 {
            if other.col > self.col {
                return Some(Direction::Right)
            }
            return Some(Direction::Left)
        }        

        if other.col > self.col {
            if other.row > self.row {
                return Some(Direction::UpRight)
            }
            return Some(Direction::DownRight)
        }

        if other.row > self.row {
            return Some(Direction::UpLeft)
        }
        return Some(Direction::DownLeft)

        // Self -> Other direction
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
    type Err = anyhow::Error;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "R" => Ok(Direction::Right),
            "L" => Ok(Direction::Left),
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            _ => Err(anyhow!("Unknown char?"))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Command {
    pub direction: Direction,
    pub steps: usize
}

impl FromStr for Command {
    type Err = anyhow::Error;
    fn from_str(value: &str) -> Result<Self, Self::Err> {

        let direction: String = value.chars().take(1).collect::<String>();
        let count: String = value.chars().skip(2).collect::<String>();
        let count = usize::from_str(&count)?;

        Ok(Self {
            direction: Direction::from_str(&direction)?,
            steps: count
        })
    }
}

#[derive(Debug, Clone)]
pub struct Game {
    pub tail_path: Vec<Position>,
    pub head_path: Vec<Position>,
    pub head_coverage: HashSet<Position>,
    pub tail_coverage: HashSet<Position>,
    pub rx: Receiver<Command>,
    pub head: Position,
    pub tail: Position
}


impl Game {

    pub fn new(rx: Receiver<Command>) -> Self {
        Self {
            tail_path: vec![Position::default()],
            head_path: vec![Position::default()],
            head_coverage: HashSet::from_iter([Position::default()]),
            tail_coverage: HashSet::from_iter([Position::default()]),
            rx,
            head: Position::default(),
            tail: Position::default()
        }
    }

    pub fn accept(&mut self, command: Command) {
        let Command { direction, steps } = command;

        for _ in 1..=steps {
            self.head += direction.vector();
            self.head_coverage.insert(self.head);
            if let Some(direction_for_tail) = self.tail.two_steps_in_direction(&self.head) {
                self.tail += direction_for_tail.vector();
                self.tail_coverage.insert(self.tail);
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
    let game = Arc::new(Mutex::new(Game::new(rx)));

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

    let size = game.lock().unwrap().tail_coverage.len();
    size
}



fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", solve_part1(input));
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
    }
}