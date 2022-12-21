pub use day_20::*;

pub const INPUT: &str = include_str!("input.txt");

pub fn get_input(s: &str) -> Mixer {
    let isizes = 
    s
    .lines()
    .map_while(|x| x.parse::<isize>().ok()).collect::<Vec<_>>();
    Mixer::new(&isizes)
}

pub fn solve(mixer: &mut Mixer) -> isize {
    mixer.run();
    let v1 = mixer.get_after(0, 1000).unwrap();
    let v2 = mixer.get_after(0, 2000).unwrap();
    let v3 = mixer.get_after(0, 3000).unwrap();
    v1 + v2 + v3
}

pub fn main() {
    
    // let mut mixer = Mixer::new(&[1, 2, -3, 3, -2, 0, 4]);
    let mut mixer = get_input(INPUT);
    println!("Solution Part 1: {}", solve(&mut mixer));
}
