use std::collections::HashMap;

use day_15::*;


fn main() {
    println!("Part 1: {}", solve_part1(include_str!("input.txt"), 2000000));
}


pub fn solve_part1(s: &str, row: usize) -> usize {
    let (_, sensor_to_beacon_map) = ClosestBeaconMap::parse(s).unwrap();

    // let mut seen_on_row = HashMap::new();
    // sensor_to_beacon_map
    for (sensor, closest_beacon) in sensor_to_beacon_map.iter() {
        let max_distance = sensor.distance(closest_beacon);
        // positions look like (row, 0), (row, 1)



    }


    // println!("map: {:#?}", map);
    0
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_smol_part1() {
        let s = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

        assert_eq!(super::solve_part1(s, 10), 26);
    }
}