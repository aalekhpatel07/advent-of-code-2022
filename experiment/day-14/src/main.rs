pub use day_14::*;
use indicatif::ProgressBar;


fn main() {
    let s = include_str!("input.txt");
    println!("Part 1: {}", solve_part1(s));
    println!("Part 2: {}", solve_part2(s));
}


pub fn solve_part1(s: &str) -> usize {
    let mut cave = Cave::parse(s).unwrap().1;

    // How straight and deep are we willing to go before
    // we conclude this is an endless void?
    let max_streak: usize = 1_000; 


    'sand: loop {
        let mut trajectory = cave.get_trajectory();
        while let Some((_point, _current_streak)) = trajectory.next() {
            if _current_streak > max_streak {
                break 'sand;
            }
        }
        cave.stabilize_sand(trajectory.position);
    }
    cave.sand.len()
}

pub fn solve_part2(s: &str) -> usize {
    let mut cave = Cave::parse(s).unwrap().1;
    cave.include_bottom_floor = true;
    cave.floor_left_most = Some(-500);
    cave.floor_right_most = Some(2000);

    let progress_bar = ProgressBar::new_spinner();

    loop {
        let mut trajectory = cave.get_trajectory();
        let mut steps_taken: usize = 0;
        progress_bar.inc(1);
        progress_bar.set_message(format!("Elapsed: {:.4} s / Speed: {:.4} (stabilizations/sec)", progress_bar.elapsed().as_secs_f64(), progress_bar.per_sec()));
        while let Some((_point, _)) = trajectory.next() {
            steps_taken += 1;
        }
        cave.stabilize_sand(trajectory.position);
        if steps_taken == 0 {
            break;
        }
    }
    progress_bar.finish_and_clear();
    cave.sand.len()
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_smol() {
        let s = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
        assert_eq!(solve_part1(s), 24);
        assert_eq!(solve_part2(s), 93);
    }

    #[test]
    fn test_big() {
        let s = include_str!("input.txt");

        assert_eq!(solve_part1(s), 638);
        assert_eq!(solve_part2(s), 31_722);

    }
}