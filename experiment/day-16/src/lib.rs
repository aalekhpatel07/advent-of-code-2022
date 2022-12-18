#![allow(unused)]

mod parse;

pub use parse::*;
use rayon::prelude::*;


use std::{collections::{HashMap, HashSet}, hash::Hash};


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Graph {
    pub(crate) to: HashMap<u8, Vec<u8>>,
    pub flow_rates: HashMap<u8, isize>,
    pub non_zero_flow_indices: Vec<u8>
}


pub type APSP = HashMap<u8, HashMap<u8, isize>>;

// There's less than 63 nodes, so we can use a i64 to represent a set of indices
// where the only bits set are the indices of those nodes.
pub type State = i64;


impl Graph {
    pub fn nodes(&self) -> impl Iterator<Item=u8> + '_ {
        self.to.keys().cloned().into_iter()
    }
    pub fn mask(&self, node: &u8) -> i64 {
        1 << *node
    }
    pub fn edges(&self) -> impl Iterator<Item=(&u8, &u8, isize)> + '_ {
        self.to.iter().flat_map(move |(from, to)| {
            to.iter().map(move |to| (from, to, self.flow_rates.get(from).unwrap().clone()))
        })
    }
    pub fn flow_of(&self, node: &u8) -> Option<isize> {
        self.flow_rates.get(node).cloned()
    }

    /// Straightforward Floyd-Warshall implementation for all pairs shortest path algorithm.
    pub fn all_pairs_shortest_paths(&self) -> APSP {

        let mut distances = HashMap::new();

        for node1 in self.to.keys() {
            for node2 in self.to.keys() {
                distances.insert((node1, node2), isize::MAX);
            }
        }

        for (from, to, _) in self.edges() {
            distances.insert((&from, &to), 1isize);
        }

        for node in self.to.keys() {
            distances.insert((node, node), 0isize);
        }

        for k in self.to.keys() {
            for i in self.to.keys() {
                for j in self.to.keys() {
                    let dist_ik = distances.get(&(i, k)).unwrap();
                    let dist_kj = distances.get(&(k, j)).unwrap();
                    if dist_ik == &isize::MAX || dist_kj == &isize::MAX {
                        continue;
                    }
                    let dist = dist_ik + dist_kj;
                    if dist < *distances.get(&(i, j)).unwrap() {
                        distances.insert((i, j), dist);
                    }
                }
            }
        }
        
        let mut results = HashMap::new();
        for ((start, end), distance) in distances.into_iter() {
            results.entry(*start).or_insert_with(HashMap::new).insert(*end, distance);
        }

        results
    }


    pub fn visualize_all_pair_shortest_paths(&self) {
        let shortest_paths = self.all_pairs_shortest_paths();

        let mut nodes = self.to.keys().map(|x| x.clone()).collect::<Vec<_>>();
        nodes.sort();
        println!("    \t{}", nodes.iter().map(|n| format!("{}", n)).collect::<Vec<String>>().join("\t"));

        for node1 in nodes.iter() {
            print!("{} ", node1);
            let mut distances = shortest_paths.get(&(node1.clone())).unwrap().into_iter().collect::<Vec<_>>();
            distances.sort_by_key(|&d| d.0);

            for (_, dist) in distances.iter() {

                print!("\t{}", dist);
            }
            println!();
        }
    }

    /// Thanks to [JuniorBirdman1115's Reddit post],
    /// the key insight is to store the visited state
    /// as a bitset (and since there are less than 63 nodes)
    /// we can use a i64 to store it.
    /// 
    /// [JuniorBirdman1115's Reddit post]: https://www.reddit.com/r/adventofcode/comments/zn6k1l/comment/j0oo5a9/
    pub fn visit<'a>(
        &self, 
        current_node: u8, 
        budget: i64,
        state: State,
        distances: &APSP,
        flow: i64,
        answer: &'a mut HashMap<State, i64>
    ) {

        // Update our cache if a better flow is achieved.
        answer
        .entry(state)
        .and_modify(|e| *e = flow.max(*e))
        .or_insert(flow.max(0));

        // For each node that has a non-zero flow,
        // see if we can visit that.
        self
        .non_zero_flow_indices
        .iter()
        .for_each(|next_node| {

            let distance = *distances.get(&current_node).unwrap().get(&next_node).unwrap() as i64;

            // If we choose to go here, then we have to travel the shortest distance to get there
            // and then turn the valve on, using up `(distance + 1)` time.
            let new_budget = budget - (distance + 1);

            let mask = self.mask(&next_node);
            let unvisited = (state & mask) == 0;

            // If not already visited and have budget to move, take that path and record the best flow
            // along it.
            if unvisited && new_budget >= 0 {
                let flow_from_neighbor = self.flow_of(&next_node).unwrap() as i64;
                self.visit(*next_node, new_budget, state | mask, distances, flow + (new_budget * flow_from_neighbor), answer);
            }
        });
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_parse_graph() {
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

        let (rem, graph) = parse_graph(s).unwrap();
        assert_eq!(rem, "");
        assert_eq!(graph.to.len(), 10);
    }
}