
use std::collections::VecDeque;

/// Given a character between 'a'-'z'
/// embed that into a u32 where the only
/// single bit set is somewhere between 
/// the first and the 26th bit from the right,
/// (i.e. \in {1 = 2 ** 0, 2, 4, 8, ..., 2 ** 25})
#[inline]
pub fn encode(c: char) -> u32 {
    let after_offset = u32::from(c) - u32::from('a');
    1 << after_offset
}

#[derive(Debug)]
pub struct MarkerProcessor {
    items: VecDeque<char>,
    state: u32
}

impl Default for MarkerProcessor {
    fn default() -> Self {
        Self::new()
    }
}


impl MarkerProcessor {
    #[inline]
    pub fn new() -> Self {
        Self { items: vec![].into(), state: 0 }
    }

    #[inline]
    pub fn refresh_state(&mut self) {
        self.state = 0;
        for item in self.items.iter() {
            self.state |= encode(*item);
        }
    }
}


fn solve(contents: &str, window_size: usize) -> usize {
    let mut processor = MarkerProcessor::new();
    
    let mut counter: usize = 0;

    for c in contents.chars() {
        if processor.items.len() >= window_size {
            let ones = processor.state.count_ones() as usize;
            if ones == window_size {
                return counter;
            }
            processor.items.pop_front();
        }

        processor.items.push_back(c);
        counter += 1;
        processor.refresh_state();
    }

    counter
}


fn main() {
    let inputs = include_str!("inputs.txt");
    let part1_answer = solve(inputs, 4);
    let part2_answer = solve(inputs, 14);
    println!("Part 1: {}", part1_answer);
    println!("Part 2: {}", part2_answer);
}


#[cfg(test)]
mod tests {

    #![allow(dead_code)]
    use super::*;
    use test_case::test_case;

    #[test_case('z', 33554432 ; "z encodes to 2 ** 25 (i.e. upper boundary)")]
    #[test_case('a', 1 ; "a encodes to 2 ** 0 (i.e. lower boundary)")]
    fn test_encode(c: char, value: u32) {
        assert_eq!(encode(c), value);
    }

    #[test_case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19, 14 ; "big test case #0")]
    #[test_case("bvwbjplbgvbhsrlpgdmjqwftvncz", 23, 14 ; "big test case #1")]
    #[test_case("nppdvjthqldpwncqszvftbrmjlhg", 23, 14 ; "big test case #2")]
    #[test_case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29, 14 ; "big test case #3")]
    #[test_case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjwnznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 26, 14 ; "big test case #4")]
    #[test_case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 4 ; "smol test case #0")]
    #[test_case("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 4 ; "smol test case #1")]
    #[test_case("nppdvjthqldpwncqszvftbrmjlhg", 6, 4 ; "smol test case #2")]
    #[test_case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 4 ; "smol test case #3")]
    #[test_case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjwnznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 11, 4 ; "smol test case #4")]
    fn test_find_marker(input: &str, marker_offset: usize, window_size: usize) {
        assert_eq!(solve(input, window_size), marker_offset);
    }

}