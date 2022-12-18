use std::cmp::Reverse;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::collections::BinaryHeap;

use day_16::Graph;
use indicatif::ProgressBar;

fn main() {
    println!("Part 1: {}", solve_part1(&get_input_graph(include_str!("input.txt"))));
}

pub fn get_input_graph(s: &str) -> Graph {
    let (_, graph) = day_16::parse_graph(s).unwrap();
    graph
}

pub fn solve_part1(graph: &Graph) -> isize {
    let distances = graph.all_pairs_shortest_paths();
    let mut unseen_nodes = graph.nodes().collect::<HashSet<_>>();
    unseen_nodes.remove(&0);

    for node in graph.nodes() {
        if !graph.non_zero_flow_indices.contains(&node) {
            unseen_nodes.remove(&node);
        }
    }

    let mut cache = HashMap::new();
    graph.best_flow_under(
        &distances, 
        30, 
        0, 
        unseen_nodes,
        &mut cache,
    );
    
    let vals = cache.into_iter().collect::<Vec<_>>();
    vals.iter().max_by_key(|(_, v)| *v).unwrap().1
}


#[cfg(test)]
mod tests {
    use day_16::Graph;
    use super::*;

    fn get_smol_input() -> Graph {
        let s = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
        get_input_graph(s)
    }

    #[test]
    fn big_part1() {
        let graph = get_smol_input();
        // WTF this doesn't work on test input but works in the given input!!
        // :sus:
        assert_eq!(1940, solve_part1(&get_input_graph(include_str!("input.txt"))));

    }
}