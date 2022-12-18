use nom::{IResult, sequence::tuple, bytes::complete::tag, character::complete::{space0, alpha1, space1, digit1}, combinator::{map, opt}, multi::{many1, separated_list1, many0, separated_list0}, branch::alt};
use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tunnels {
    pub from: String,
    pub flow_rate: isize,
    pub to: Vec<String>
}


impl<T> From<T> for Graph 
where
    T: IntoIterator<Item=Tunnels>
{
    fn from(s: T) -> Self {
        let mut hmap = HashMap::new();
        let mut flow_map = HashMap::new();
        let all_tunnels = s.into_iter().collect::<Vec<_>>();
        let mut starting_nodes = all_tunnels.iter().map(|t| t.from.clone()).collect::<Vec<_>>();
        starting_nodes.sort();

        let mut indices = starting_nodes.iter().enumerate().map(|(idx, _)| idx as u8).collect::<Vec<_>>();
        let mut non_zero_indices = Vec::new();

        for (idx, tunnels) in all_tunnels.iter().enumerate() {
            let from = tunnels.from.clone();
            let from_index = starting_nodes.binary_search(&from).unwrap() as u8;

            let neighbors = tunnels
            .to
            .iter()
            .map(|t| starting_nodes.binary_search(&t).unwrap() as u8)
            .collect::<Vec<_>>();

            hmap.insert(from_index, neighbors);
            // hmap.insert(idx as u8, tunnels.to.iter().map(|t| all_tunnels.binary_search_by_key(&t, |z| z.from.clone())).collect::<Vec<_>>());
            flow_map.insert(from_index, tunnels.flow_rate);
            if tunnels.flow_rate > 0 {
                non_zero_indices.push(from_index);
            }
        }

        Graph {
            to: hmap,
            flow_rates: flow_map,
            non_zero_flow_indices: non_zero_indices,
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