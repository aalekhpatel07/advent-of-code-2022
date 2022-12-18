#![allow(unused)]

mod parse;

use indicatif::ProgressBar;
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

impl Graph {
    pub fn nodes(&self) -> impl Iterator<Item=u8> + '_ {
        self.to.keys().cloned().into_iter()
    }
    pub fn edges(&self) -> impl Iterator<Item=(&u8, &u8, isize)> + '_ {
        self.to.iter().flat_map(move |(from, to)| {
            to.iter().map(move |to| (from, to, self.flow_rates.get(from).unwrap().clone()))
        })
    }
    pub fn flow_of(&self, node: &u8) -> Option<isize> {
        self.flow_rates.get(node).cloned()
    }

    #[inline(always)]
    pub fn sorted_nodes(&self) -> Vec<u8> {
        let mut nodes = self.nodes().collect::<Vec<_>>();
        nodes.sort();
        nodes
    }

    pub fn convert_hashset_to_u64_bitset(&self, hset: &HashSet<u8>) -> u64 {
        let mut bitset = 0;
        for node in hset {
            bitset |= 1 << self.sorted_nodes().binary_search(node).unwrap();
        }
        bitset
    }

    pub fn best_flow_under(
        &self, 
        apsp: &APSP,
        remaining_time: isize,
        current_node: u8,
        mut unseen_nodes: HashSet<u8>,
        cache: &mut HashMap<(isize, u8, u64), isize>
    ) {

        let current_node_distances = apsp.get(&current_node).unwrap();
        if cache.contains_key(&(remaining_time, current_node.clone(), self.convert_hashset_to_u64_bitset(&unseen_nodes))) {
            // println!("Found in cache!");
            return;
        }

        // If no time remaining, we cannot add any more flow.
        if remaining_time <= 0 {
            cache
            .entry((remaining_time, current_node.clone(), self.convert_hashset_to_u64_bitset(&unseen_nodes)))
            .and_modify(|e| *e = 0.max(*e))
            .or_insert(0);
            return;
        }

        // We try to visit every node possible within our remaining time
        // and we return the maximum flow we can get if we turn that valve on.

        let mut unseen_nodes_cp = unseen_nodes.clone();
        unseen_nodes_cp.remove(&current_node);

        let candidate_nodes = unseen_nodes
        .iter()
        .filter(|&node| {
            let distance = *current_node_distances.get(node).unwrap();
            (distance + 1) as isize <= remaining_time
        })
        .collect::<Vec<_>>();

        if candidate_nodes.len() == 0 {
            // No nodes to visit, we cannot add any more flow.
            cache
            .entry((remaining_time, current_node.clone(), self.convert_hashset_to_u64_bitset(&unseen_nodes)))
            .and_modify(|e| *e = 0.max(*e))
            .or_insert(0);
            return;
        }

        let best_value = 
        candidate_nodes
        .iter()
        .map(|&candidate_node| {
            let distance = current_node_distances.get(candidate_node).unwrap();
            let mut unseen_nodes_cp = unseen_nodes_cp.clone();
            unseen_nodes_cp.remove(candidate_node);

            self.best_flow_under(
                apsp, 
                remaining_time - (distance + 1) as isize, 
                candidate_node.to_owned(), 
                unseen_nodes_cp.clone(),
                cache,
            );

            let cache_key = (remaining_time - (distance + 1) as isize, candidate_node.to_owned(), self.convert_hashset_to_u64_bitset(&unseen_nodes_cp));
            let flow = cache.get(&cache_key).unwrap();
            let value = *flow + self.flow_of(&current_node).unwrap() * (remaining_time as isize);

            value
        })
        .max()
        .unwrap_or(0);

        cache
        .entry((remaining_time, current_node.clone(), self.convert_hashset_to_u64_bitset(&unseen_nodes)))
        .and_modify(|e| *e = best_value.max(*e))
        .or_insert(best_value);

    }


    /// Straightforward Floyd-Warshall implementation for all pairs shortest_path algorithm.
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
        
        // let mut as_list = distances.into_iter().collect::<Vec<_>>();
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
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Node {
    pub(crate) name: String,
    pub(crate) flow_rate: usize
}

impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
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
        println!("{:#?}", graph);
    }
}