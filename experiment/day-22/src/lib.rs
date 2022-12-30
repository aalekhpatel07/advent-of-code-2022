mod parse;
pub use parse::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Open,
    Occupied,
    Blank
}

impl Default for Tile {
    fn default() -> Self {
        Self::Blank
    }
}

#[derive(Debug, Clone)]
pub struct Board<const N: usize, const M: usize> {
    pub board: [[Tile; N]; M]
}


#[derive(Debug, Clone, Copy)]
pub enum Step {
    CounterClockwise,
    Clockwise,
    Forward(u64)
}


#[derive(Debug, Clone)]
pub struct Instructions {
    pub instructions: Vec<Step>
}


// #[derive(Debug, Clone)]
// pub struct Game<const N: usize, const M: usize> {
//     pub board: Board<N, M>,
//     pub instructions: Instructions,
// }

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn delta(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
    pub fn clockwise(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    pub fn counter_clockwise(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    pub fn score(&self) -> usize {
        match self {
            Direction::Up => 3,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Right => 0,
        }
    }
}

impl Default for Direction {
    fn default() -> Self {
        Self::Right
    }
}


pub type Coordinate = (usize, usize);


#[derive(Debug, Clone)]
pub struct Game<const N: usize, const M: usize> {
    pub board: Board<N, M>,
    pub instructions: Instructions,
    pub current_position: Coordinate,
    pub current_direction: Direction,
}


impl<const N: usize, const M: usize> Game<N, M> {
    pub fn new(board: Board<N, M>, instructions: Instructions) -> Self {

        let (index, _) = board.board[0].iter().enumerate().find(|&c| {
            match c.1 {
                Tile::Open => true,
                _ => false
            }
        }).expect("No open cell on the first row.");

        Self {
            board,
            instructions,
            current_position: (0, index),
            current_direction: Direction::default()
        }
    }

    #[inline(always)]
    pub fn wrapping_shift(&self, index: Coordinate, direction: Direction) -> Coordinate {
        let delta = direction.delta();

        let (row, col) = index;

        let mut updated_row = row as isize + delta.0;
        let mut updated_col = col as isize + delta.1;

        if updated_row == -1 {
            updated_row = M as isize - 1;
        } else if updated_row == M as isize {
            updated_row = 0;
        }

        if updated_col == -1 {
            updated_col = N as isize - 1;
        } else if updated_col == N as isize {
            updated_col = 0;
        }

        (updated_row as usize, updated_col as usize)
    }


    pub fn step(&mut self, step: &Step) {
        match step {
            Step::Clockwise => {
                self.current_direction = self.current_direction.clockwise();
            },
            Step::CounterClockwise => {
                self.current_direction = self.current_direction.counter_clockwise();
            },
            Step::Forward(units) => {
                if *units == 0 {
                    return;
                }

                // Now we're guaranteed to move in that direction. So first skip all blank tiles.
                let current_direction = self.current_direction;

                let mut steps_remaining = *units as isize;

                while steps_remaining > 0 {
                    let mut next_position = self.wrapping_shift(self.current_position, self.current_direction);
                    match self.board.board[next_position.0][next_position.1] {
                        Tile::Occupied => {
                            break;
                        },
                        Tile::Open => {
                            self.current_position = next_position;
                            steps_remaining -= 1;
                        },
                        Tile::Blank => {
                            // Skip all blank tiles.
                            loop {
                                next_position = self.wrapping_shift(self.current_position, current_direction);
                                if self.board.board[next_position.0][next_position.1] != Tile::Blank {
                                    break;
                                }
                                self.current_position = next_position;
                            }
                            steps_remaining -= 1;
                        }
                    }
                }
            }
        }
    }

    pub fn play(&mut self) -> (Coordinate, Direction) {
        for step in &self.instructions.instructions.clone() {
            self.step(step);
        }
        ((self.current_position.0 + 1, self.current_position.1 + 1), self.current_direction)
    }

}