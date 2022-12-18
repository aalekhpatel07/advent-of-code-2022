mod parse;
use std::cmp::Ordering;
use std::fmt::Display;
use std::hash::Hash;
pub use parse::*;
use rayon::collections::vec_deque::Iter;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use std::ops::Deref;
use std::collections::{HashMap, HashSet};


#[derive(Clone, Copy, PartialEq, Eq, Ord)]
pub struct Position {
    pub x: isize,
    pub y: isize
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Position) -> Option<Ordering> {
        self.y
        .partial_cmp(&other.y)
        .or_else(|| self.x.partial_cmp(&other.x))
    }
}


impl Position {
    #[inline(always)]
    pub fn distance(&self, other: &Position) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub gradient: isize,
    pub y_intercept: isize,
}


impl Line {
    pub fn new(start: Position, end: Position) -> Self {
        let gradient = (end.y - start.y) / (end.x - start.x);
        let y_intercept = start.y - gradient * start.x;
        Self { gradient, y_intercept }
    }
    pub fn intersection_point(&self, other: &Self) -> Option<Position> {
        if self.gradient == other.gradient {
            return None;
        }
        let x = (other.y_intercept - self.y_intercept) / (self.gradient - other.gradient);
        let y = self.gradient * x + self.y_intercept;
        Some(Position { x, y })
    }
}


pub type Sensor = Position;
pub type Beacon = Position;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClosestBeaconMap {
    pub sensor_to_beacon_map: HashMap<Sensor, Beacon>,
    pub occupied_positions: HashSet<Position>
}

impl Deref for ClosestBeaconMap {
    type Target = HashMap<Sensor, Beacon>;

    fn deref(&self) -> &Self::Target {
        &self.sensor_to_beacon_map
    }
}

impl ClosestBeaconMap {
    #[inline(always)]
    pub fn is_available(&self, position: &Position) -> bool {
        !self.occupied_positions.contains(position)
    }


    pub fn get_boundary_lines(&self, sensor: &Sensor) -> [Line; 4] {
        let closest_beacon = self.sensor_to_beacon_map.get(sensor).unwrap();
        let max_distance = sensor.distance(closest_beacon);

        [
            Line::new(
                Position { x: sensor.x, y: sensor.y + max_distance + 1 }, Position { x: sensor.x + max_distance + 1, y: sensor.y }
            ),
            Line::new(
                Position { x: sensor.x, y: sensor.y + max_distance + 1 }, Position { x: sensor.x - max_distance - 1, y: sensor.y }
            ),
            Line::new(
                Position { x: sensor.x - max_distance - 1, y: sensor.y }, Position { x: sensor.x, y: sensor.y - max_distance - 1 }
            ),
            Line::new(
                Position { x: sensor.x + max_distance + 1, y: sensor.y }, Position { x: sensor.x, y: sensor.y - max_distance - 1 }
            )
        ]
    }

    pub fn get_intersection_points(&self, sensor1: &Sensor, sensor2: &Sensor) -> Vec<Position>{
        let lines = self.get_boundary_lines(sensor1);
        let other_lines = self.get_boundary_lines(sensor2);
        let mut intersection_points = Vec::new();
        for line in lines.iter() {
            for other_line in other_lines.iter() {
                if let Some(point) = line.intersection_point(other_line) {
                    intersection_points.push(point);
                }
            }
        }
        intersection_points.sort();
        intersection_points.dedup();
        intersection_points
    }

    pub fn get_positions_outside_boundary(&self, sensor: &Sensor) -> HashSet<Position> {
        let closest_beacon = self.sensor_to_beacon_map.get(sensor).unwrap();
        let max_distance = sensor.distance(closest_beacon);

        let highest_row = sensor.y + max_distance;
        let lowest_row = sensor.y - max_distance;

        let highest_col = sensor.x + max_distance;
        let lowest_col = sensor.y - max_distance;

        let mut boundary = (lowest_row..=highest_row)
        .filter_map(move |row| {
            let lower_bound = (sensor.x - max_distance) + (sensor.y - row).abs();
            let upper_bound = (sensor.x + max_distance) - (sensor.y - row).abs();
            match lower_bound <= upper_bound {
                true => Some((lower_bound, upper_bound, row)),
                false => None
            }
        })
        .flat_map(|(lower_bound, upper_bound, row)| {
            // (lower_bound..=upper_bound)
            // // .filter(move |&col| {
            // //     !self.is_available(&Position { x: col, y: row })
            // // })
            // .map(move |col| {
            //     let pos = Position { x: col, y: row };
            //     pos
            // })

            vec![
                Position { x: lower_bound - 1, y: row },
                Position { x: upper_bound + 1, y: row },
            ].into_iter()

        })
        .collect::<HashSet<Position>>();

        boundary.insert(Position { x: sensor.x, y: highest_row + 1});
        boundary.insert(Position { x: sensor.x, y: lowest_row - 1});
        boundary.insert(Position { y: sensor.y, x: lowest_col - 1});
        boundary.insert(Position { y: sensor.y, x: highest_col + 1});

        boundary

    }

    pub fn get_covered_positions_per_beacon(&self, sensor: &Sensor) -> HashSet<Position> {
        let closest_beacon = self.sensor_to_beacon_map.get(sensor).unwrap();
        let max_distance = sensor.distance(closest_beacon);

        let highest_row = sensor.y + max_distance;
        let lowest_row = sensor.y - max_distance;
        // let highest_col = closest_beacon.x + max_distance;
        // let lowest_col = closest_beacon.x - max_distance;

        (lowest_row..=highest_row)
        .filter_map(move |row| {
            let lower_bound = (sensor.x - max_distance) + (sensor.y - row).abs();
            let upper_bound = (sensor.x + max_distance) - (sensor.y - row).abs();
            match lower_bound <= upper_bound {
                true => Some((lower_bound, upper_bound, row)),
                false => None
            }
        })
        .flat_map(|(lower_bound, upper_bound, row)| {
            (lower_bound..=upper_bound)
            // .filter(move |&col| {
            //     !self.is_available(&Position { x: col, y: row })
            // })
            .map(move |col| {
                let pos = Position { x: col, y: row };
                pos
            })
        })
        .collect::<HashSet<Position>>()
    }

    pub fn get_available_along_row(
        &self, 
        lower_bound: isize, 
        upper_bound: isize,
        row: isize,
    ) -> HashSet<Position> {

        match lower_bound <= upper_bound {
            true => {
                (lower_bound..=upper_bound)
                .filter(move |&col| {
                    self.is_available(&Position { x: col, y: row })
                })
                .map(move |col| {
                    Position { x: col, y: row }
                })
                .collect::<HashSet<Position>>()
            },
            false => return HashSet::new()
        }
    }

    pub fn is_free_position(&self, pos: &Position) -> bool {
        self
        .sensor_to_beacon_map
        .par_iter()
        .all(|(sensor, closest_beacon)| {
            sensor.distance(pos) > sensor.distance(closest_beacon)
        })
    }

    pub fn get_unique_positions_along_row_where_beacon_definitely_doesnt_exist(
        &self,
        row: isize
    ) -> HashSet<Position> {

        self
        .sensor_to_beacon_map
        .par_iter()
        .map(|(sensor, closest_beacon)| {
            let max_distance = sensor.distance(closest_beacon);
            // positions look like (0, row), (1, row), (k, row)

            // d(sensor, (k, 10)) <= max_distance
            // => d((sx, sy), (k, 10)) <= max_distance
            // => |(sx - k)| + |sy - 10| <= max_distance
            // => |sy - 10| <= max_distance - |sx - k|
            // => |sx - k| <= max_distance - |sy - 10|
            // => -(max_distance - |sy - 10|) <= sx - k <= max_distance - |sy - 10|
            // => k <= sx + max_distance - |sy - 10|
            // => k >= sx - max_distance + |sy - 10|

            let lower_bound = (sensor.x - max_distance) + (sensor.y - row).abs();
            let upper_bound = (sensor.x + max_distance) - (sensor.y - row).abs();
            
            self.get_available_along_row(lower_bound, upper_bound, row).into_iter()
        })
        .fold(
            || HashSet::new(),
            |m1, m2| {
                m2.fold(m1, |mut acc, p| {
                    acc.insert(p);
                    acc
                })
            })
            .reduce(
            || HashSet::new(),
            |m1, m2| {
                m2.iter().fold(m1, |mut acc, &v| {
                    acc.insert(v);
                    acc
                })
            }
        )
    }
    // pub fn 
}


impl Hash for Sensor {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_range() {
        let beacon_map = ClosestBeaconMap {
            sensor_to_beacon_map: vec![
            (Position { x: 2, y: 18 }, Position { x: -2, y: 15 }),
            (Position { x: 9, y: 16 }, Position { x: 10, y: 16 }),
            ].into_iter().collect(),
            occupied_positions: vec![
                Position { x: 2, y: 18 },
                Position { x: -2, y: 15 },
                Position { x: 9, y: 16 },
                Position { x: 10, y: 16 },
            ].into_iter().collect()
        };

        let x = beacon_map.get_unique_positions_along_row_where_beacon_definitely_doesnt_exist(100);
        println!("{:#?}", x);

    }

    #[test]
    fn get_positions_outside_boundary() {
        let beacon_map = ClosestBeaconMap {
            sensor_to_beacon_map: vec![
                (Position { x: 8, y: 7 }, Position { x: 2, y: 10 }),
            ].into_iter().collect(),
            occupied_positions: vec![
                Position { x: 2, y: 10 },
            ].into_iter().collect()
        };
        let res = beacon_map.get_positions_outside_boundary(&Position { x: 8, y: 7});

        let expected: HashSet<Position> = vec![
            (8,-3),
            (7,-2),
            (9,-2),
            (10,-1),
            (6,-1),
            (5,0),
            (11,0),
            (4,1),
            (12,1),
            (13,2),
            (3,2),
            (2,3),
            (14,3),
            (15,4),
            (1,4),
            (0,5),
            (16,5),
            (-1,6),
            (17,6),
            (-3,7),
            (18,7),
            (-2,7),
            (17,8),
            (-1,8),
            (16,9),
            (0,9),
            (15,10),
            (1,10),
            (14,11),
            (2,11),
            (13,12),
            (3,12),
            (4,13),
            (12,13),
            (5,14),
            (11,14),
            (6,15),
            (10,15),
            (9,16),
            (7,16),
            (8,17),
        ].into_iter().map(|(x, y)| Position {x, y}).collect();
        assert_eq!(res, expected);
    }
    #[test]
    fn get_covered_positions_per_beacon() {

        let beacon_map = ClosestBeaconMap {
            sensor_to_beacon_map: vec![
                (Position { x: 8, y: 7 }, Position { x: 2, y: 10 }),
            ].into_iter().collect(),
            occupied_positions: vec![
                Position { x: 2, y: 10 },
            ].into_iter().collect()
        };

        let covered = beacon_map.get_covered_positions_per_beacon(&Position { x: 8, y: 7 });
        let mut as_list = covered.into_iter().collect::<Vec<_>>();
        as_list.sort();
        println!("{:#?}", as_list);

    }


    #[test]
    fn boundary_lines() {

        let beacon_map = ClosestBeaconMap {
            sensor_to_beacon_map: vec![
                (Position { x: 8, y: 7 }, Position { x: 2, y: 10 }),
            ].into_iter().collect(),
            occupied_positions: vec![
                Position { x: 2, y: 10 },
            ].into_iter().collect()
        };

        let res = beacon_map.get_boundary_lines(&Position { x: 8, y: 7 });
        println!("{:#?}", res);

    }

    #[test]
    fn get_intersection_points() {

    }
    // #[test]
    // fn line_intersection() {
    //     let l1 = Line { gradient: -1, y_intercept: 25};
    //     let l2 = Line { gradient: -1, y_intercept: 5};
    //     let l3 = Line { gradient: 1, y_intercept: 9};
    //     let l4 = Line { gradient: 1, y_intercept: -11};

    //     // assert_eq!(l1.intersection_point(&l2), None);
    //     // assert_eq!(l1.intersection_point(&l3), Some(Position { x: 8, y: 17}));
    // }
}