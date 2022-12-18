use day_15::*;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};


fn main() {
    println!("Part 1: {}", solve_part1(include_str!("input.txt"), 2_000_000));
    let pos = solve_part2(include_str!("input.txt"), 4_000_000).unwrap();
    println!("Part 2: {}", pos.x * 4_000_000 + pos.y);
}


/// Idea:
/// 
/// For each sensor, obtain the leftmost and rightmost positions on the given row.
/// Collect all of them into a set and return the count.
pub fn solve_part1(s: &str, row: isize) -> usize {
    let (_, sensor_to_beacon_map) = ClosestBeaconMap::parse(s).unwrap();
    sensor_to_beacon_map.get_unique_positions_along_row_where_beacon_definitely_doesnt_exist(row).len()
}

/// Idea:
/// 
/// Since exactly one position exists where
/// the distress beacon could be, it must lie pretty close outside the boundary
/// of one of the sensors. If it lies a little too far, then there exists a position between the
/// boundary and that point that is also a valid position for the beacon, but that cannot happen.
/// 
/// Now, we could naively check all points along the boundary, but there is a better way.
/// Since the distress beacon must lie outside boundaries of all the sensors, in particular,
/// it must lie just outside the intersection of some two sensor boundaries. We can obtain the
/// points of intersection of boundaries of two sensors by solving a system of linear equations
/// of four line segments with gradients +- 1. 
/// 
/// A pair of sensors may have at most 8 intersection points since every line on the boundary
/// intersects exactly 2 other lines in the other sensor's boundary. For each of those 8 points,
/// we check if it is a valid position for the beacon. If it is, we return it.
/// 
/// Therefore we end up checking O(|sensor| * |sensor| * 8) points and for each point we verify
/// against every sensor that it lies outside the boundaries. Therefore this solution is O(|sensor|^3).
/// 
pub fn solve_part2(
    s: &str,
    upper_bound: isize,
) -> Option<Position> {
    
    let (_, sensor_to_beacon_map) = ClosestBeaconMap::parse(s).unwrap();
    let sensors = sensor_to_beacon_map.keys().collect::<Vec<_>>();
    let sensors_cp = sensors.clone();

    sensors
    .iter()
    .flat_map(|&sensor| {
        sensors_cp.iter().map(|&s| (*sensor, *s))
    }).collect::<Vec<_>>()
    .par_iter()
    .find_map_first(|(sensor1, sensor2)| {
        match sensor1 == sensor2 {
            true => None,
            false => {
                sensor_to_beacon_map
                .get_intersection_points(sensor1, sensor2)
                .par_iter()
                .find_first(|&pos| (0 <= pos.x && pos.x <= upper_bound && 0 <= pos.y && pos.y <= upper_bound) && sensor_to_beacon_map.is_free_position(&pos))
                .and_then(|x| Some(*x))
            }
        }
    })
}



#[cfg(test)]
mod tests {
    use day_15::Position;


    #[test]
    fn test_smol_part1() {
        let s = "Sensor at x=8, y=7: closest beacon is at x=2, y=10";
        assert_eq!(super::solve_part1(s, 10), 12);
    }

    #[test]
    fn test_big_part2() {
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
        assert_eq!(super::solve_part2(s, 20), Some(Position { x: 14, y: 11}));
    }


    #[test]
    fn test_big_part1() {
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
    #[test]
    fn test_input_part1() {
        assert_eq!(super::solve_part1(include_str!("input.txt"), 2_000_000), 5_125_700);
    }

    #[test]
    fn test_input_part2() {
        let sol = super::solve_part2(include_str!("input.txt"), 4_000_000).unwrap();
        assert_eq!(4_000_000 * sol.x + sol.y, 11379394658764);
    }
}