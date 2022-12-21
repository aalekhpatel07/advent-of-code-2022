pub use day_20::*;

pub const INPUT: &str = include_str!("input.txt");

pub fn get_input(s: &str, scale_factor: Value) -> Mixer {
    s
    .lines()
    .map_while(
        |x| x.parse::<Value>().ok()
    )
    .enumerate()
    .map(|(idx, x)| Number(x * scale_factor, idx))
    .collect::<Vec<_>>()
}


pub fn main() {
    
    let mut mixer = get_input(INPUT, 1);
    println!("Solution Part 1: {}", solve(&mut mixer, 1));

    let mut mixer = get_input(INPUT, 811_589_153);
    println!("Solution Part 2: {}", solve(&mut mixer, 10));
}
