mod parse;
use std::{collections::{HashSet, HashMap}, cmp::Reverse};

use indicatif::ProgressBar;
pub use parse::*;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Left,
    Right,
    Down
}

#[derive(Debug, Clone)]
pub struct Directions {
    inner: Vec<Direction>,
    index: usize
}

impl Iterator for Directions {
    type Item = Direction;
    fn next(&mut self) -> Option<Self::Item> {
        let prev = self.inner[self.index];
        self.index = (self.index + 1) % self.inner.len();
        Some(prev)
    }
}

impl From<Vec<Direction>> for Directions {
    fn from(inner: Vec<Direction>) -> Self {
        Self { inner, index: 0 }
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rock {
    pub col_min: u8,
    pub col_max: u8,
    pub row_min: u8,
    pub row_max: u8,
    pub filled: Vec<(u8, u8)>,
    pub empty_space_required: HashMap<Direction, Vec<(i8, i8)>>
}


#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum RockKind {
    Minus,
    Plus,
    MirrorL,
    Vertical,
    Square
}

impl Rock {
    /// Measure them from bottom-left of the 4x4 block
    /// with going right implying an increase in (.1)
    /// and going down implying an increase in (.0)
    pub fn new(kind: RockKind) -> Self {
        match kind {
            RockKind::Minus => {
                Self {
                    col_min: 0,
                    col_max: 3,
                    row_min: 0,
                    row_max: 0,
                    filled: vec![(0, 0), (0, 1), (0, 2), (0, 3)],
                    empty_space_required: HashMap::from_iter(vec![
                        (Direction::Left, vec![(0, -1)]),
                        (Direction::Right, vec![(0, 4)]),
                        (Direction::Down, vec![(-1, 0), (-1, 1), (-1, 2), (-1, 3)])
                    ].into_iter())
                }
            },
            RockKind::Plus => {
                Self {
                    col_min: 0,
                    col_max: 2,
                    row_min: 0,
                    row_max: 2,
                    filled: vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
                    empty_space_required: HashMap::from_iter(vec![
                        (Direction::Left, vec![(1, -1), (0, 0), (2, 0)]),
                        (Direction::Right, vec![(1, 3), (0, 2), (2, 2)]),
                        (Direction::Down, vec![(0, 0), (-1, 1), (0, 2)])
                    ].into_iter())
                }
            },
            RockKind::MirrorL => {
                Self {
                    col_min: 0,
                    col_max: 2,
                    row_min: 0,
                    row_max: 2,
                    filled: vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)],
                    empty_space_required: HashMap::from_iter(vec![
                        (Direction::Left, vec![(0, -1), (1, 1), (2, 1)]),
                        (Direction::Right, vec![(0, 3), (1, 3), (2, 3)]),
                        (Direction::Down, vec![(-1, 0), (-1, 1), (-1, 2)])
                    ].into_iter())
                }
            },
            RockKind::Vertical => {
                Self {
                    col_min: 0,
                    col_max: 0,
                    row_min: 0,
                    row_max: 3,
                    filled: vec![(0, 0), (1, 0), (2, 0), (3, 0)],
                    empty_space_required: HashMap::from_iter(vec![
                        (Direction::Left, vec![(0, -1), (1, -1), (2, -1), (3, -1)]),
                        (Direction::Right, vec![(0, 1), (1, 1), (2, 1), (3, 1)]),
                        (Direction::Down, vec![(-1, 0)])
                    ].into_iter())
                }
            },
            RockKind::Square => {
                Self {
                    col_min: 0,
                    col_max: 1,
                    row_min: 0,
                    row_max: 1,
                    filled: vec![(0, 0), (0, 1), (1, 0), (1, 1)],
                    empty_space_required: HashMap::from_iter(vec![
                        (Direction::Left, vec![(0, -1), (1, -1)]),
                        (Direction::Right, vec![(0, 2), (1, 2)]),
                        (Direction::Down, vec![(-1, 0), (-1, 1)])
                    ].into_iter())
                }
            }
        }
    }

    // pub fn can_move(&self, direction: Direction) -> bool {
    //     let positions = self.empty_space_required.get(&direction).unwrap();

    //     // match direction {
    //     //     Direction::Left => {

    //     //     },
    //     //     Direction::Right => {

    //     //     },
    //     //     Direction::Down => {

    //     //     }
    //     // };
    //     false
    // }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RockKindIterator {
    inner: Vec<RockKind>,
    index: usize
}

impl RockKindIterator {
    pub fn new() -> Self {
        Self {
            inner: vec![
                RockKind::Minus,
                RockKind::Plus,
                RockKind::MirrorL,
                RockKind::Vertical,
                RockKind::Square
            ],
            index: 0
        }
    }
}


impl Iterator for RockKindIterator {
    type Item = RockKind;
    fn next(&mut self) -> Option<Self::Item> {
        let prev = self.inner[self.index];
        self.index = (self.index + 1) % self.inner.len();
        Some(prev)
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rocks {
    rock_kind_iterator: RockKindIterator
}

impl Rocks {
    pub fn new() -> Self {
        Self {
            rock_kind_iterator: RockKindIterator::new()
        }
    }
}

impl Iterator for Rocks {
    type Item = (Rock, RockKind);
    fn next(&mut self) -> Option<Self::Item> {
        self
        .rock_kind_iterator
        .next()
        .and_then(|kind| Some((Rock::new(kind), kind)))
    }
}

#[derive(Debug, Clone)]
pub struct ActiveRock {
    pub rock: Rock,
    pub kind: RockKind,
    pub bottom_left: (isize, isize),
}

#[derive(Debug, Clone)]
pub struct Cave {
    rocks: Rocks,
    directions: Directions,
    pub width: usize,
    existing_filled: HashSet<(isize, isize)>,
    active_rock: Option<ActiveRock>,
    spawn_position: Option<(isize, isize)>,
    rocks_stabilized: usize,
    pub quiet: bool,
    pub log_spawn: bool
}

impl Cave {
    pub fn new(directions: Directions, width: usize) -> Self {
        Self {
            rocks: Rocks::new(),
            directions,
            width,
            existing_filled: HashSet::new(),
            active_rock: None,
            spawn_position: Some((3, 2)),
            rocks_stabilized: 0,
            quiet: true,
            log_spawn: true,
        }
    }

    pub fn height(&self) -> usize {
        self.existing_filled.iter().map(|(x, _)| *x).max().unwrap_or(0) as usize + 1
    }

    pub fn run(&mut self, max_stabilized_rocks: usize) {
        let progress_bar = ProgressBar::new(max_stabilized_rocks as u64);
        loop {
            if self.active_rock.is_none() {
                // No active rock. Spawn a new one.
                let (rock, kind) = self.rocks.next().unwrap();
                self.active_rock = Some(ActiveRock {
                    rock,
                    kind,
                    bottom_left: self.spawn_position.unwrap()
                });

                if self.log_spawn || !self.quiet {
                    println!("A new rock begins falling!:");
                    println!("{}", self);
                }
            }

            let jet_direction = self.directions.next().unwrap();
            let mut active_rock = self.active_rock.clone().unwrap();
            
            let empty_spaces_required = active_rock.rock.empty_space_required.get(&jet_direction).unwrap();
            
            let all_are_unoccupied = empty_spaces_required
            .iter()
            .all(
                |(delta_row, delta_col)| {
                    let position = (active_rock.bottom_left.0 + *delta_row as isize, active_rock.bottom_left.1 + *delta_col as isize);
                    !self.is_occupied(position)
                }
            );

            if all_are_unoccupied {
                // now move the active rock left/right.
                match jet_direction {
                    Direction::Left => {
                        active_rock.bottom_left = (active_rock.bottom_left.0, active_rock.bottom_left.1 - 1);
                    },
                    Direction::Right => {
                        active_rock.bottom_left = (active_rock.bottom_left.0, active_rock.bottom_left.1 + 1);
                    },
                    _ => unreachable!("Should not be able to move down due to jet.")
                }
                self.active_rock = Some(active_rock.clone());
                if !self.quiet {
                    println!("Jet of gas pushes rock {:#?}:", jet_direction);
                    println!("{}", self);
                }
            } else {
                if !self.quiet {
                    println!("Jet of gas pushes rock {:#?}, but nothing happens:", jet_direction);
                    println!("{}", self);
                }
            }

            // Now time to move the rock down one unit, if we can.
            let can_move_down = 
                active_rock
                .rock
                .empty_space_required
                .get(&Direction::Down)
                .unwrap()
                .iter()
                .all(
                    |(delta_row, delta_col)| {
                        let position = (active_rock.bottom_left.0 + *delta_row as isize, active_rock.bottom_left.1 + *delta_col as isize);
                        !self.is_occupied(position)
                    }
                );

            if can_move_down {
                active_rock.bottom_left = (active_rock.bottom_left.0 - 1, active_rock.bottom_left.1);
                self.active_rock = Some(active_rock.clone());
                if !self.quiet {
                    println!("Rock falls 1 unit:");
                    println!("{}", self);
                }
            } else {
                // We cannot move down. This rock is now stabilized.
                // Add it to the existing filled.
                self.rocks_stabilized += 1;
                progress_bar.inc(1);
                active_rock.rock.filled.iter().for_each(|(p_row, p_col)| {
                    let position = (active_rock.bottom_left.0 + *p_row as isize, active_rock.bottom_left.1 + *p_col as isize);
                    self.existing_filled.insert(position);
                });
                // Tricky stuff. The jet could potentially maneuver the rock
                // much lower than the previous max.
                self.spawn_position = Some(
                    (
                        self.existing_filled.iter().max_by_key(|x| x.0).unwrap().0 + 4, 
                        2
                    )
                );
                self.active_rock = None;

                if !self.quiet {
                    println!("Rock falls 1 unit, causing it to come to rest:");
                    println!("{}", self);
                }
            }

            if self.rocks_stabilized == max_stabilized_rocks {
                break;
            }
        }
    }

    pub fn is_occupied(&self, (row, col): (isize, isize)) -> bool {
        if col < 0 || col >= self.width as isize || row < 0 {
            return true;
        }
        self.existing_filled.contains(&(row, col))
    }


}

use std::fmt::Display;

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        let lowest_col = 0 as isize;
        let highest_col = self.width as isize;
        let lowest_row = 0;
        let highest_row = {
            // First check the existing stabilized ones.
            let mut highest = 
                self
                .existing_filled
                .iter()
                .max_by_key(
                    |(row, _)| *row
                )
                // .map(|(row, _)| *row)
                .unwrap_or(&(0, 0)).0;
            
            // Now if there's an active_rock, check that too.
            if let Some(active_rock) = &self.active_rock {
                let (row, _) = active_rock.bottom_left;
                let row = row as isize;
                highest = highest.max(row + active_rock.rock.row_max as isize);
            }


            highest
        };

        let mut results = vec![
            format!("+{}+", "-".repeat(self.width)),
        ];

        for row in lowest_row..=highest_row {
            let mut line = String::new();
            line.push('|');
            for col in lowest_col..highest_col {
                if self.is_occupied((row, col)) {
                    line.push('#');
                } else {
                    line.push('.');
                }
            }
            line.push('|');
            results.push(line);
        }

        
        // Get indices of an active rock, if exist.
        let active_rock_indices = self.active_rock.as_ref().map(|active_rock| {
            let (row, col) = active_rock.bottom_left;
            let row = row as isize;
            let col = col as isize;
            active_rock.rock.filled.iter().map(|(r, c)| (row + *r as isize, col + *c as isize)).collect::<Vec<_>>()
        }).unwrap_or(vec![]);

        results.reverse();
        let results_len = results.len();
        for (row, col) in active_rock_indices {
            let row = row as usize;
            let col = col as usize;
            results[results_len - row - 2].replace_range((col+1)..(col+2), "@");
        }


        let contents = results.join("\n");

        write!(f, "{}\n", contents)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_directions() {
        let directions = vec![Direction::Left, Direction::Right];
        let mut directions = super::Directions::from(directions);
        assert_eq!(directions.next(), Some(Direction::Left));
        assert_eq!(directions.next(), Some(Direction::Right));
        assert_eq!(directions.next(), Some(Direction::Left));
        assert_eq!(directions.next(), Some(Direction::Right));
        assert_eq!(directions.next(), Some(Direction::Left));
        assert_eq!(directions.next(), Some(Direction::Right));
    }

    #[test]
    fn test_rocks() {
        let mut rocks = Rocks::new();

        assert_eq!(rocks.next(), Some((Rock::new(RockKind::Minus), RockKind::Minus)));
        assert_eq!(rocks.next(), Some((Rock::new(RockKind::Plus), RockKind::Plus)));
        assert_eq!(rocks.next(), Some((Rock::new(RockKind::MirrorL), RockKind::MirrorL)));
        assert_eq!(rocks.next(), Some((Rock::new(RockKind::Vertical), RockKind::Vertical)));
        assert_eq!(rocks.next(), Some((Rock::new(RockKind::Square), RockKind::Square)));
        assert_eq!(rocks.next(), Some((Rock::new(RockKind::Minus), RockKind::Minus)));
        assert_eq!(rocks.next(), Some((Rock::new(RockKind::Plus), RockKind::Plus)));
        assert_eq!(rocks.next(), Some((Rock::new(RockKind::MirrorL), RockKind::MirrorL)));
        assert_eq!(rocks.next(), Some((Rock::new(RockKind::Vertical), RockKind::Vertical)));
        assert_eq!(rocks.next(), Some((Rock::new(RockKind::Square), RockKind::Square)));
        assert_eq!(rocks.next(), Some((Rock::new(RockKind::Minus), RockKind::Minus)));
    }

    #[test]
    fn test_cave() {
        let directions = vec![Direction::Left, Direction::Right];
        let directions = super::Directions::from(directions);
        let mut cave = Cave::new(directions, 7);
        assert_eq!(cave.width, 7);
        let hset: HashSet<(isize, isize)> = HashSet::from_iter(vec![
            (0, 2),
            (0, 3),
            (0, 4),
            (0, 5),
            (1, 3),
            (2, 2),
            (2, 3),
            (2, 4),
            (3, 3),
        ]);
        cave.existing_filled = hset;

        let active_rock = ActiveRock {
            rock: Rock::new(RockKind::MirrorL),
            kind: RockKind::MirrorL,
            bottom_left: (7, 3)
        };
        cave.active_rock = Some(active_rock);

        println!("{}", cave);
    }

    #[test]
    fn test_cave_rock_stabilize() {
        let s = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        let directions = parse_direction(s).unwrap().1;
        let directions = super::Directions::from(directions);
        let mut cave = Cave::new(directions, 7);
        cave.quiet = true;
        cave.log_spawn = false;
        cave.run(2022);
        // let s = format!("{}", cave);
        // println!("{}", s);
        println!("{:#?}", cave.existing_filled.iter().max_by_key(|x| x.0).unwrap());
    }
}