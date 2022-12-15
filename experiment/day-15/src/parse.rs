use nom::{IResult, sequence::tuple, character::{complete::digit1, complete::space0}, combinator::{map_res, map, opt}, bytes::complete::tag, multi::{many0, many1}, branch::alt};

use crate::{Position, ClosestBeaconMap, Beacon, Sensor};


pub trait Parse {
    fn parse(input: &str) -> IResult<&str, Self>
    where
        Self: Sized;
}


pub fn signed(s: &str) -> IResult<&str, String> {
    alt((
        map(
            tuple((tag("-"), digit1)), 
            |(t, d): (&str, &str)| format!("{}{}", t, d)
        ),
        map(digit1, String::from)
    ))(s)
}


impl Parse for Position {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            tuple((
                tag("x="),
                signed,
                tag(", y="),
                signed
            )),
            |(_, x, _, y): (_, String, _, String)| {
                Position {
                    x: x.parse::<isize>().unwrap(),
                    y: y.parse::<isize>().unwrap()
                }
            }
        )
        (input)
    }    
}


impl Parse for ClosestBeaconMap {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
    many0(
                map(
                    tuple((
                        tag("Sensor at "),
                        Sensor::parse,
                        tag(": closest beacon is at "),
                        Beacon::parse,
                        opt(tag("\n"))
                    )),
                    |(_, s, _, b, _): (_, Sensor, _, Beacon, _)| {
                        (s, b)
                    }
                )
            ),
            |v: Vec<(Sensor, Beacon)>| {
                ClosestBeaconMap(v.into_iter().collect())
            }
        )(input)
    }
}


// #[cfg(tests)]
mod tests {
    use super::*;
    // use test_case::test_case;


    #[test]
    fn test_sensor_beacon_map_parse() {
        let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16";

        let expected = ClosestBeaconMap(vec![
            (Position { x: 2, y: 18 }, Position { x: -2, y: 15 }),
            (Position { x: 9, y: 16 }, Position { x: 10, y: 16 }),
        ].into_iter().collect());

        let (rem, observed) = ClosestBeaconMap::parse(input).unwrap();
        assert_eq!(expected, observed);
        assert_eq!(rem, "");

    }
}