use std::{collections::HashMap, fmt::{Debug, Display}, cmp::Ordering};
use indicatif::{ProgressBar, ProgressIterator};

type IndexMap<T> = HashMap<T, usize>;
type IndexLookupMap<T> = HashMap<usize, T>;


#[derive(Debug, Clone, Eq)]
pub struct Mixer {
    pub gps: Vec<isize>,
    pub index_map: IndexMap<isize>,
    pub index_lookup_map: IndexLookupMap<isize>
}

impl PartialEq for Mixer {
    fn eq(&self, other: &Self) -> bool {

        let other_items = (0..other.gps.len()).map(|x| *other.index_lookup_map.get(&x).unwrap()).collect::<Vec<_>>();

        (0..self.gps.len())
        .into_iter()
        .any(|step_size| self.rotate_left(step_size) == other_items)
    }
}

impl PartialEq<Vec<isize>> for Mixer {
    fn eq(&self, other: &Vec<isize>) -> bool {
        let other_mixer = Mixer::new(other);
        self == &other_mixer
    }
}

impl Mixer {
    pub fn new(list: &[isize]) -> Self {
        let mut index_map = IndexMap::new();
        let mut index_lookup_map = IndexLookupMap::new();
        for (i, x) in list.iter().enumerate() {
            index_map.insert(*x, i);
            index_lookup_map.insert(i, *x);
        }
        Mixer {
            gps: list.iter().cloned().collect(),
            index_map,
            index_lookup_map
        }
    }

    pub fn get_after(&self, item: isize, count: usize) -> Option<isize> {
        let index_of_item = *self.index_map.get(&item).expect("Item doesn't exist in index map");
        let index_of_desired_item = (index_of_item + count) % self.gps.len();
        self.index_lookup_map.get(&index_of_desired_item).cloned()
    }

    pub fn rotate_left(&self, steps: usize) -> Vec<isize> {
        let mut items = (0..self.gps.len()).map(|x| *self.index_lookup_map.get(&x).unwrap()).collect::<Vec<_>>();
        let drained = items.drain(0..steps).collect::<Vec<_>>();
        items.extend(drained);
        items
    }

    pub fn step_right(&mut self, item: isize, count: usize) {
        let index_of_item = *self.index_map.get(&item).expect("Item doesn't exist in index map");
        let new_index_of_item = (index_of_item + count) % self.gps.len();

        let mut starting_index = new_index_of_item;
        let mut starting_item = item;
        let mut index_map_cache = IndexMap::new();
        index_map_cache.insert(starting_item, starting_index);
        let mut index_lookup_map_cache = IndexLookupMap::new();
        index_lookup_map_cache.insert(starting_index, starting_item);

        loop {
            let previous_item = *self.index_lookup_map.get(&starting_index).expect("Item doesn't exist in index lookup map");
            if previous_item == item {
                break;
            }
            let previous_index = *self.index_map.get(&previous_item).expect("Item doesn't exist in index map");

            starting_item = previous_item;
            starting_index = if previous_index as isize == 0 { self.gps.len() - 1 } else { (previous_index as isize - 1) as usize};

            index_lookup_map_cache.insert(starting_index, starting_item);
            index_map_cache.insert(starting_item, starting_index);
        }

        for (item, index) in index_map_cache.iter() {
            self.index_map.insert(*item, *index);
        }
        for (index, item) in index_lookup_map_cache.iter() {
            self.index_lookup_map.insert(*index, *item);
        }

    }

    pub fn step(&mut self, item: isize) {
        match item.partial_cmp(&0) {
            Some(Ordering::Equal) => {
                // Do nothing.
            },
            Some(Ordering::Greater) => {
                self.step_right(item, item.abs() as usize);
            },
            Some(Ordering::Less) => {
                self.step_right(item, self.gps.len() - (item.abs() as usize) - 1);
            },
            None => {
                unreachable!("isize should be comparable to 0. wtf");
            }
        }
    }

    pub fn run(&mut self) {
        let items_to_step = self.gps.clone().into_iter().collect::<Vec<_>>();

        items_to_step
        .into_iter()
        .progress()
        .for_each(
            |item| self.step(item)
        );
    }

}


impl Display for Mixer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let items = (0..self.gps.len()).map(|index| *self.index_lookup_map.get(&index).unwrap()).collect::<Vec<_>>();
        write!(f, "{}", items.into_iter().map(|x| format!("{}", x)).collect::<Vec<_>>().join(", "))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        let gps = Mixer::new(&[1, 2, -3, 3, -2, 0, 4]);
        let expected = vec![3, -2, 0, 4, 1, 2, -3];
        assert_eq!(gps.rotate_left(3), expected);
    }

    #[test]
    fn test_mixer() {
        let mut gps = Mixer::new(&[1, 2, -3, 3, -2, 0, 4]);
        gps.step(1);
        assert_eq!(gps, vec![2, 1, -3, 3, -2, 0, 4]);
        gps.step(2);
        assert_eq!(gps, vec![1, -3, 2, 3, -2, 0, 4]);
        gps.step(-3);
        assert_eq!(gps, vec![1, 2, 3, -2, -3, 0, 4]);
        gps.step(3);
        assert_eq!(gps, vec![1, 2, -2, -3, 0, 3, 4]);
        gps.step(-2);
        assert_eq!(gps, vec![1, 2, -3, 0, 3, 4, -2]);
        gps.step(0);
        assert_eq!(gps, vec![1, 2, -3, 0, 3, 4, -2]);
        gps.step(4);
        assert_eq!(gps, vec![1, 2, -3, 4, 0, 3, -2]);

        assert_eq!(gps.get_after(4, 21), Some(4));
        assert_eq!(gps.get_after(4, 22), Some(0));
        assert_eq!(gps.get_after(4, 17), Some(-2));

    }
}