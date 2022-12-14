mod parse;
use std::cmp::Ordering;
use itertools::Itertools;
use itertools::EitherOrBoth::{Both, Left, Right};

pub use parse::*;


#[derive(Debug, PartialEq, Eq, Clone, Ord)]
pub enum Packet {
    One(usize),
    Many(Box<List>)
}


#[derive(Debug, PartialEq, Eq, Clone, Ord)]
pub struct List {
    pub list: Vec<Packet>
}

pub struct ListIterator<'a> {
    list: &'a List,
    index: usize
}

impl<'a> Iterator for ListIterator<'a> {
    type Item = &'a Packet;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.list.list.len() {
            let item = &self.list.list[self.index];
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}


impl<'a> IntoIterator for &'a List {
    type Item = &'a Packet;
    type IntoIter = ListIterator<'a>;
    fn into_iter(self) -> Self::IntoIter {
        ListIterator {
            list: self,
            index: 0
        }
    }
}


impl PartialOrd for List {
    fn partial_cmp(&self, _other: &List) -> Option<Ordering> {
        unreachable!(
            "Just a no-op to make the compiler happy. It won't let me Ord without PartialOrd for List but the actual PartialOrd for Packet doesn't really use the List's PartialOrd anyways."
        );
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Packet) -> Option<Ordering> {

        match (self, other) {
            (Packet::One(a), Packet::One(b)) => a.partial_cmp(b),
            (Packet::One(a), m) if matches!(m, Packet::Many(_)) => {
                Packet::Many(Box::new(List { list: vec![Packet::One(*a)] })).partial_cmp(m)
            },
            (m, Packet::One(a)) if matches!(m, Packet::Many(_)) => {
                m.partial_cmp(&Packet::Many(Box::new(List { list: vec![Packet::One(*a)] })))
            },
            (Packet::Many(a), Packet::Many(b)) => {
                let mut conclusion = None;

                a
                .into_iter()
                .zip_longest(
                    b.into_iter()
                )
                .for_each(|pair| {
                    if conclusion.is_some() && conclusion != Some(Ordering::Equal) {
                        return;
                    }
                    match pair {
                        Both(left, right) => {
                            // Both left and right items are present and we haven't yet concluded anything.
                            // Compare them.
                            conclusion = left.partial_cmp(&right);
                        },
                        Left(_) => {
                            // Right ran out of items.
                            conclusion = Some(Ordering::Greater);
                        },
                        Right(_) => {
                            // Left ran out of items.
                            conclusion = Some(Ordering::Less);
                        }
                    };
                });
                conclusion
            },
            _ => unreachable!("Happy now, rustc?")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;

    use test_case::test_case;


    #[test_case("[1,1,3,1,1]", "[1,1,5,1,1]", Some(Ordering::Less))]
    #[test_case("[[1],[2,3,4]]", "[[1],4]", Some(Ordering::Less))]
    #[test_case("[9]", "[[8,7,6]]", Some(Ordering::Greater))]
    #[test_case("[[4,4],4,4]", "[[4,4],4,4,4]", Some(Ordering::Less))]
    #[test_case("[7,7,7,7]", "[7,7,7]", Some(Ordering::Greater))]
    #[test_case("[]", "[3]", Some(Ordering::Less))]
    #[test_case("[[[]]]", "[[]]", Some(Ordering::Greater))]
    #[test_case("[1,[2,[3,[4,[5,6,7]]]],8,9]", "[1,[2,[3,[4,[5,6,0]]]],8,9]", Some(Ordering::Greater))]
    fn test_packet_compare(a: &str, b: &str, expected_ordering: Option<Ordering>) {
        let (_, p1) = Packet::parse(a).unwrap();
        let (_, p2) = Packet::parse(b).unwrap();
        assert_eq!(p1.partial_cmp(&p2), expected_ordering);
    }
}