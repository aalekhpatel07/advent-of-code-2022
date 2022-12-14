mod parse;
use std::fmt::Display;
use std::collections::HashSet;

pub use parse::*;


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RockSegment {
    pub points: Vec<Point>
}

impl RockSegment {
    pub fn rocks(&self) -> impl IntoIterator<Item=Point> {
        let mut buffer = vec![];

        self
        .points
        .iter()
        .take(self.points.len() - 1)
        .zip(self.points.iter().skip(1))
        .for_each(|(start, end)| {
            match (start.x == end.x, start.y == end.y) {
                (true, false) => {
                    if start.y < end.y {
                        buffer.extend((start.y..=end.y).map( |y| Point { x: start.x, y }));
                    } else {
                        buffer.extend((end.y..=start.y).map( |y| Point { x: start.x, y }).into_iter());
                    }
                },
                (false, true) => {
                    if start.x < end.x {
                        buffer.extend((start.x..=end.x).map( |x| Point { x, y: start.y }).into_iter());
                    } else {
                        buffer.extend((end.x..=start.x).map( |x| Point { x, y: start.y }).into_iter());
                    }
                },
                _ => unreachable!("Only handling vertical/horizontal segments for now.")
            }
        });
        buffer.into_iter()
    }
}


#[derive(Debug, Clone)]
pub struct Trajectory {
    pub obstacles: HashSet<Point>,
    pub position: Point,
    pub current_streak: usize,
}


type PointWithStreak = (Point, usize);

impl Iterator for Trajectory {
    type Item = PointWithStreak;

    fn next(&mut self) -> Option<Self::Item> {

        // If our current position is occupied means we can't even enter the cave.
        if self.obstacles.contains(&self.position) {
            return None;
        }

        // Try going down first.
        if !self.obstacles.contains(&Point { x: self.position.x, y: self.position.y + 1 }) {
            // we could go down. lets gooo.
            self.position.y += 1;
            self.current_streak += 1;
            return Some((self.position, self.current_streak));
        }

        // We couldn't go down because we saw a rock or another sand grain.
        // Try going left.
        if !self.obstacles.contains(&Point { x: self.position.x - 1, y: self.position.y + 1 }) {
            // we could go left. lets gooo.
            self.position.x -= 1;
            self.position.y += 1;
            self.current_streak = 1;
            return Some((self.position, self.current_streak));
        }

        // We couldn't go left because we saw a rock or another sand grain.
        // Try going right.
        if !self.obstacles.contains(&Point { x: self.position.x + 1, y: self.position.y + 1 }) {
            // we could go left. lets gooo.
            self.position.x += 1;
            self.position.y += 1;
            self.current_streak = 1;
            return Some((self.position, self.current_streak));
        }


        // We couldn't go anywhere. So we've stabilized.
        // Make sure to update our obstacles with this point's position.
        self.obstacles.insert(self.position);

        None
    }
}







#[derive(Debug, Clone)]
pub struct Cave {
    pub rock_segments: Vec<RockSegment>,
    pub sand: Vec<Point>,
    pub include_bottom_floor: bool,
    pub floor_left_most: Option<isize>,
    pub floor_right_most: Option<isize>
}

impl Cave {

    pub fn rocks(&self) -> impl Iterator<Item=Point> + '_ {

        self
        .rock_segments
        .iter()
        .flat_map(|segment| segment.rocks())
    }

    pub fn bottom_floor(&self) -> impl Iterator<Item=Point> + '_ {
        
        let y = self.bottom_right().y + 2;
        // let left_most = self.rocks().map(|p| p.x).min().unwrap();
        // let right_most = self.rocks().map(|p| p.x).max().unwrap();

        (self.floor_left_most.unwrap()..=self.floor_right_most.unwrap()).map(move |x| Point { x, y })
    }

    pub fn height(&self) -> usize {
        0
    }

    pub fn width(&self) -> usize {
       0 
    }

    pub fn top_left(&self) -> Point {
        let mut left_most = self.rocks().map(|p| p.x).min().unwrap();
        if self.include_bottom_floor {
            // Now the left and right are virtually infinite.
            left_most = 450;
        }
        Point { x: left_most, y: 0 }
    }
    pub fn bottom_right(&self) -> Point {

        let mut right_most = self.rocks().map(|p| p.x).max().unwrap();
        if self.include_bottom_floor {
            // Now the left and right are virtually infinite.
            right_most = 550;
        }
        let bottom_most = self.rocks().map(|p| p.y).max().unwrap();
        Point { x: right_most, y: bottom_most }
    }

    pub fn get_trajectory(&self) -> Trajectory {

        let mut obstacles: HashSet<Point> = (self.sand.clone().into_iter()).chain(self.rocks()).collect();

        if self.include_bottom_floor {
            obstacles.extend(self.bottom_floor());
        }

        Trajectory {
            obstacles,
            position: Point { x: 500, y: 0 },
            current_streak: 0,
        }
    }

    pub fn stabilize_sand(&mut self, sand: Point) {
        self.sand.push(sand);
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let mut rocks = self.rocks().collect::<HashSet<_>>();
        let top_left = self.top_left();
        let mut bottom_right = self.bottom_right();

        if self.include_bottom_floor {
            bottom_right.y += 2;
            rocks.extend(self.bottom_floor());
        }


        for y in top_left.y..=bottom_right.y {
            for x in top_left.x..=bottom_right.x {
                if rocks.contains(&Point { x, y }) {
                    write!(f, "#")?;
                } else if self.sand.contains(&Point { x, y }) {
                    write!(f, "o")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }

        writeln!(f)
    }
}