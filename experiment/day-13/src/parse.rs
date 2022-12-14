use crate::{List, Packet};
use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1, combinator::map,
    multi::separated_list0, sequence::tuple, IResult,
};

pub trait Parse {
    fn parse(input: &str) -> IResult<&str, Self>
    where
        Self: Sized;
}

impl Parse for List {
    fn parse(input: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
        map(
            tuple((tag("["), separated_list0(tag(","), Packet::parse), tag("]"))),
            |(_, list, _)| List { list },
        )(input)
    }
}

impl Parse for Packet {
    fn parse(input: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
        alt((
            map(digit1, |s: &str| Packet::One(s.parse::<usize>().unwrap())),
            map(List::parse, |p| Packet::Many(Box::new(p))),
        ))(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("[[[]]]")]
    #[test_case("[[1],[2,3,4]]")]
    fn test_parse(s: &str) {
        let _ = Packet::parse(s).unwrap();
    }
}
