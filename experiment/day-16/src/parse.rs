use nom::{IResult, sequence::tuple, bytes::complete::tag, character::complete::{space0, alpha1, space1, digit1}, combinator::{map, opt}, multi::{many1, separated_list1, many0, separated_list0}, branch::alt};
use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tunnels {
    pub from: String,
    pub flow_rate: usize,
    pub to: Vec<String>
}


impl<T> From<T> for Graph 
where
    T: IntoIterator<Item=Tunnels>
{
    fn from(s: T) -> Self {
        let mut hmap = HashMap::new();
        let mut flow_map = HashMap::new();

        for tunnels in s {
            hmap.insert(tunnels.from.clone(), tunnels.to);
            flow_map.insert(tunnels.from, tunnels.flow_rate);
        }

        Graph {
            to: hmap,
            flow_rates: flow_map
        }
    }
}


pub fn parse_tunnels(s: &str) -> IResult<&str, Tunnels> {
    map(
        tuple((
            tag("Valve"),
            space1,
            alpha1,
            space1,
            tag("has flow rate="),
            digit1,
            alt((
                tag("; tunnels lead to valves "),
                tag("; tunnel leads to valve ")
            )),
            separated_list0(tag(", "), alpha1)
        )),
        |(_, _, from_valve, _, _, flow_rate, _, to_valves): (_, _, &str, _, _, &str, _, Vec<&str>)| {
            Tunnels {
                from: from_valve.to_string(),
                flow_rate: flow_rate.parse().unwrap(),
                to: to_valves.into_iter().map(String::from).collect()
            }
        }
    )(s)
}

pub fn parse_graph(s: &str) -> IResult<&str, Graph> {
    map(
        separated_list0(tag("\n"), parse_tunnels),
        Graph::from
    )(s)
}

#[cfg(test)]
mod test {
    use crate::{parse_tunnels, Tunnels};

    #[test]
    fn parse() {
        let s = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB";
        let expected = Tunnels {
            from: "AA".to_string(),
            flow_rate: 0,
            to: vec!["DD".to_string(), "II".to_string(), "BB".to_string()]
        };
        let (rem, observed) = parse_tunnels(s).unwrap();
        assert_eq!(expected, observed);
        assert_eq!(rem, "");
    }

    #[test]
    fn parse_single_node_tunnel() {
        let s = "Valve HH has flow rate=22; tunnel leads to valve GG";
        let expected = Tunnels {
            from: "HH".to_string(),
            flow_rate: 22,
            to: vec!["GG".to_string()]
        };
        let (rem, observed) = parse_tunnels(s).unwrap();
        assert_eq!(expected, observed);
        assert_eq!(rem, "");
    }
}