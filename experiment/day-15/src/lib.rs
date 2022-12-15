mod parse;
use std::hash::Hash;
pub use parse::*;
use std::ops::Deref;
use std::collections::HashMap;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: isize,
    pub y: isize
}

impl Position {
    pub fn distance(&self, other: &Position) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
    }
}


pub type Sensor = Position;
pub type Beacon = Position;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClosestBeaconMap(HashMap<Sensor, Beacon>);

impl Deref for ClosestBeaconMap {
    type Target = HashMap<Sensor, Beacon>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


impl Hash for Sensor {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

