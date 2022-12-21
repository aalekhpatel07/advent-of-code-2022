use std::{collections::HashMap, fmt::{Debug, Display}, cmp::Ordering};
use indicatif::{ProgressBar, ProgressIterator};


pub type Value = i128;

#[derive(Debug, Clone, Copy)]
pub struct Number(pub Value, pub usize);
pub type Mixer = Vec<Number>;

pub trait AOCDay20 {
    // Move around the value at the given index.
    fn step(&mut self, current_position: usize);

    // Run the whole simulation a whole buncha times.
    fn run(&mut self, times: usize);

    // Get the value after `count` items after the given item.
    fn get_after(&self, item: Value, count: usize) -> Option<Value>;
}


impl AOCDay20 for Mixer {
    
    fn step(&mut self, current_position: usize) {
        let modulus = (self.len() as Value) - 1;
        let index = self.iter().position(|x| x.1 == current_position).unwrap();
        // gotta make sure negatives become positive.
        let new_index = (((index as Value + self.get(index).unwrap().0) % modulus) + modulus) % modulus;
        // tbh didn't think this naive solution would pass, but it did.
        let previous = self.remove(index);
        self.insert(new_index as usize, previous);
    }

    fn get_after(&self, item: Value, count: usize) -> Option<Value> {
        self
        .iter()
        .position(|x| x.0 == item)
        .map(
            |index| 
            self
            .get((index + count) % self.len())
            .unwrap()
            .0
        )
    }

    fn run(&mut self, times: usize) {
        for _ in 0..times {
            let mut current_position = 0;
            let self_iter_cp = self.clone();
            for _ in self_iter_cp.into_iter().progress() {
                self.step(current_position);
                current_position = (current_position + 1) % self.len();
            }
        }
    }
}

pub fn solve(mixer: &mut Mixer, times: usize) -> Value {
    mixer.run(times);
    let v1 = mixer.get_after(0, 1000).unwrap();
    let v2 = mixer.get_after(0, 2000).unwrap();
    let v3 = mixer.get_after(0, 3000).unwrap();
    v1 + v2 + v3
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mixer() {
        let mut mixer = vec![
            Number(1, 0),
            Number(2, 1),
            Number(-3, 2),
            Number(3, 3),
            Number(-2, 4),
            Number(0, 5),
            Number(4, 6),
        ];

        let mut mixer2 = mixer.clone().into_iter().map(|number| Number(number.0 * 811589153, number.1)).collect::<Vec<_>>();
        
        assert_eq!(3, solve(&mut mixer, 1));
        assert_eq!(1623178306, solve(&mut mixer2, 10));
        
    }
}