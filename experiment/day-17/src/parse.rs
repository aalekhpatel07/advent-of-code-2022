use nom::{IResult, multi::many1, bytes::complete::tag, combinator::map};
use crate::Direction;


pub fn parse_direction(s: &str) -> IResult<&str, Vec<Direction>> {
    many1(
        nom::branch::alt((
            map(
                tag("<"),
                |_| Direction::Left,
            ),
            map(
                tag(">"),
                |_| Direction::Right,
            ),
        ))
    )(s)
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn parse() {
        let s = "><";
        let (rem, result) = parse_direction(s).unwrap();
        assert_eq!(rem, "");
        assert_eq!(&result, &vec![Direction::Right, Direction::Left]);
    }
}