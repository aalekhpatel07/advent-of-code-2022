use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::collections::BinaryHeap;

use day_16::Graph;
use indicatif::ProgressBar;

fn main() {
    // println!("Hello, world!");
    solve_part1(&get_input_graph(include_str!("input.txt")));
}

pub fn get_input_graph(s: &str) -> Graph {
    let (_, graph) = day_16::parse_graph(s).unwrap();
    graph
}

pub fn solve_part1(graph: &Graph) -> isize {
    // graph.visualize_all_pair_shortest_paths();
    let distances = graph.all_pairs_shortest_paths();
    let unseen_nodes = graph.nodes().collect::<HashSet<_>>();
    // unseen_nodes.remove("AA");
    let mut cache = HashMap::new();
    // let progress_bar = ProgressBar::new(1_000_000_000);
    // let cache_counter = ProgressBar::new(1_000_000_000);
    let cache_key_hset = graph.convert_hashset_to_u64_bitset(&unseen_nodes);

    graph.best_flow_under(
        &distances, 
        30, 
        "AA".to_owned(), 
        unseen_nodes,
        &mut cache,
    );
    
    println!("cache: {:#?}", cache.get(&(30, "AA".to_owned(), cache_key_hset)).unwrap());
    0
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
    fn smol_part1() {
        let graph = get_smol_input();
        // graph.visualize_all_pair_shortest_paths();
        println!("{}", solve_part1(&graph));
    }
}