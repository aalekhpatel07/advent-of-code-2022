use nom::{
    bytes::complete::tag,
    character::complete::char as nom_char,
    character::complete::digit1,
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

use crate::{Cave, Point, RockSegment};

pub trait Parse {
    fn parse(s: &str) -> IResult<&str, Self>
    where
        Self: Sized;
}

impl Parse for Point {
    fn parse(s: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
        map(
            separated_pair(
                map_res(digit1, str::parse::<isize>),
                nom_char(','),
                map_res(digit1, str::parse::<isize>),
            ),
            |(x, y)| Point { x, y },
        )(s)
    }
}

impl Parse for RockSegment {
    fn parse(s: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
        map(separated_list1(tag(" -> "), Point::parse), |points| {
            RockSegment { points }
        })(s)
    }
}

impl Parse for Cave {
    fn parse(s: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
        map(
            separated_list1(tag("\n"), RockSegment::parse),
            |rock_segments| Cave {
                rock_segments,
                sand: vec![],
                include_bottom_floor: false,
                floor_left_most: None,
                floor_right_most: None,
            },
        )(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("498,4 -> 498,6 -> 496,6", 3)]
    #[test_case("503,4 -> 502,4 -> 502,9 -> 494,9", 4)]
    fn test_path(s: &str, expected_length: usize) {
        let (remaining, path) = RockSegment::parse(s).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(path.points.len(), expected_length);
    }
}
