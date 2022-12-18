use std::collections::HashMap;
use day_16::APSP;
use day_16::Graph;
use rayon::prelude::IntoParallelRefIterator;
use rayon::prelude::ParallelIterator;


fn main() {
    let graph = &get_input_graph(include_str!("input.txt"));
    let distances = graph.all_pairs_shortest_paths();
    println!("Part 1: {}", solve_part1(&graph, &distances));
    println!("Part 2: {}", solve_part2(&graph, &distances));
}

pub fn get_input_graph(s: &str) -> Graph {
    let (_, graph) = day_16::parse_graph(s).unwrap();
    graph
}


pub fn solve_part1(graph: &Graph, distances: &APSP) -> i64 {
    let mut answer = HashMap::new();
    graph.visit(0, 30, 0, &distances, 0, &mut answer);
    *answer.values().max().unwrap()
}


pub fn solve_part2(graph: &Graph, distances: &APSP) -> i64 {
    let state: i64 = 0;
    let mut answer = HashMap::new();
    graph.visit(0, 26, state, &distances, 0, &mut answer);

    let answer_cp = answer.clone();

    // Assume that the elephant and us take a disjoint path for
    // opening the valves. Moreover, we actually end up opening all the 
    // non-zero flow valves. So we can get the max of the sum of the flows
    // where the two paths are disjoint subsets of the non-zero flow valves.
    answer
    .iter()
    .flat_map(|(&n1, &n2)| {
        answer_cp.iter().map(move |(&m1, &m2)| ((n1, n2), (m1, m2)))
    }).collect::<Vec<_>>()
    .par_iter()
    // filter disjoint states.
    .filter(|((k1, _), (k2, _))| {
        (*k1 & *k2) == 0
    })
    // get sums of the flows along those paths.
    .map(|((_, v1), (_, v2))| *v1 + *v2)
    .max()
    .unwrap()
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
    fn smol() {
        let graph = get_smol_input();
        let distances = graph.all_pairs_shortest_paths();
        assert_eq!(1651, solve_part1(&graph, &distances));
        assert_eq!(1707, solve_part2(&graph, &distances));

    }

    #[test]
    fn big() {
        let graph = get_input_graph(include_str!("input.txt"));
        let distances = graph.all_pairs_shortest_paths();
        assert_eq!(solve_part1(&graph, &distances), 1940);
        assert_eq!(solve_part2(&graph, &distances), 2469);
    }
}