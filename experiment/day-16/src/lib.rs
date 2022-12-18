#![allow(unused)]

mod parse;

use indicatif::ProgressBar;
pub use parse::*;
use rayon::prelude::*;


use std::{collections::{HashMap, HashSet}, hash::Hash};


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Graph {
    pub(crate) to: HashMap<String, Vec<String>>,
    pub flow_rates: HashMap<String, usize>
}

impl Graph {
    pub fn nodes(&self) -> impl Iterator<Item=String> + '_ {
        self.to.keys().cloned().into_iter()
    }
    pub fn edges(&self) -> impl Iterator<Item=(String, String, usize)> + '_ {
        self.to.iter().flat_map(move |(from, to)| {
            to.iter().map(move |to| (from.clone(), to.clone(), self.flow_rates.get(from).unwrap().clone()))
        })
    }
    pub fn neighbors(&self, node: &str) -> impl Iterator<Item=String> + '_ {
        self.to.get(node).into_iter().flat_map(|v| v.iter().cloned())
    }
    pub fn len(&self) -> usize {
        self.to.len()
    }
    pub fn flow_of(&self, node: &str) -> Option<isize> {
        self.flow_rates.get(node).and_then(|x| Some(*x as isize))
    }


    pub fn best_flow_under(
        &self, 
        apsp: &HashMap<String, HashMap<String, usize>>,
        remaining_time: isize,
        current_node: String,
        mut unseen_nodes: HashSet<String>,
        mut cache: HashMap<(isize, String, String), isize>,
        progress_bar: ProgressBar
    ) -> (HashMap<(isize, String, String), isize>, isize) {

        let current_node_distances = apsp.get(&current_node).unwrap();


        let mut unseen_nodes_list = unseen_nodes.clone().into_iter().collect::<Vec<String>>();
        unseen_nodes_list.sort();
        if cache.contains_key(&(remaining_time, current_node.clone(), unseen_nodes_list.join(","))) {
            return (cache.clone(), *cache.get(&(remaining_time, current_node.clone(), unseen_nodes_list.join(","))).unwrap());
        }

        // If no time remaining, we cannot add any more flow.
        if remaining_time <= 0 {
            let mut unseen_nodes_list = unseen_nodes.clone().into_iter().collect::<Vec<String>>();
            unseen_nodes_list.sort();
            cache.insert((remaining_time, current_node.clone(), unseen_nodes_list.join(",")), 0);
            return (cache, 0);
        }

        // We try to visit every node possible within our remaining time
        // and we return the maximum flow we can get if we turn that valve on.

        let mut unseen_nodes_cp = unseen_nodes.clone();
        unseen_nodes_cp.remove(&current_node);


        let candidate_nodes = unseen_nodes
        .iter()
        .filter(|&node| {
            let distance = current_node_distances.get(node).unwrap();
            (distance + 1) as isize <= remaining_time
        })
        .collect::<Vec<_>>();

        if candidate_nodes.len() == 0 {
            // No nodes to visit, we cannot add any more flow.
            return (cache, 0);
        }
        let mut cache_cp = cache.clone();

        candidate_nodes
        .par_iter()
        .map(|&candidate_node| {
            let distance = current_node_distances.get(candidate_node).unwrap();
            let flow = self.flow_of(&candidate_node).unwrap();
            let mut unseen_nodes_cp = unseen_nodes_cp.clone();
            unseen_nodes_cp.remove(candidate_node);
            let (cache_computed, flow) = self.best_flow_under(
                apsp, 
                remaining_time - (distance + 1) as isize, 
                candidate_node.to_owned(), 
                unseen_nodes_cp,
                cache_cp.clone(),
                progress_bar.clone()
            );
            progress_bar.inc(1);

            let value = flow + self.flow_of(&current_node).unwrap() * (remaining_time as isize);

            let mut unseen_nodes_list = unseen_nodes.clone().into_iter().collect::<Vec<String>>();
            unseen_nodes_list.sort();
            let mut cache_cp = cache_cp.clone();
            cache_cp.insert((remaining_time, current_node.clone(), unseen_nodes_list.join(",")), value);
            (cache_cp, value)
        })
        .max_by_key(|(c, v)| *v)
        .unwrap_or((cache, 0))

    }

    /// Straightforward Floyd-Warshall implementation for all pairs shortest_path algorithm.
    pub fn all_pairs_shortest_paths(&self) -> HashMap<String, HashMap<String, usize>> {

        let mut distances = HashMap::new();

        for node1 in self.to.keys() {
            for node2 in self.to.keys() {
                distances.insert((node1.clone(), node2.clone()), usize::MAX);
            }
        }

        for (from, to, _) in self.edges() {
            distances.insert((from.clone(), to.clone()), 1usize);
        }

        for node in self.to.keys() {
            distances.insert((node.clone(), node.clone()), 0usize);
        }

        for k in self.to.keys() {
            for i in self.to.keys() {
                for j in self.to.keys() {
                    let dist_ik = distances.get(&(i.clone(), k.clone())).unwrap();
                    let dist_kj = distances.get(&(k.clone(), j.clone())).unwrap();
                    if dist_ik == &usize::MAX || dist_kj == &usize::MAX {
                        continue;
                    }
                    let dist = dist_ik + dist_kj;
                    if dist < *distances.get(&(i.clone(), j.clone())).unwrap() {
                        distances.insert((i.clone(), j.clone()), dist);
                    }
                }
            }
        }
        
        // let mut as_list = distances.into_iter().collect::<Vec<_>>();
        let mut results = HashMap::new();
        for ((start, end), distance) in distances.into_iter() {
            results.entry(start).or_insert_with(HashMap::new).insert(end, distance);
        }


        results
    }


    pub fn visualize_all_pair_shortest_paths(&self) {
        let shortest_paths = self.all_pairs_shortest_paths();
        // let mut as_list = shortest_paths.into_iter().collect::<Vec<_>>();
        // as_list.sort();

        let mut nodes = self.to.keys().map(|x| x.clone()).collect::<Vec<_>>();
        nodes.sort();
        println!("    \t{}", nodes.join("\t"));

        for node1 in nodes.iter() {
            print!("{} ", node1);
            let mut distances = shortest_paths.get(&(node1.clone())).unwrap().into_iter().collect::<Vec<_>>();
            distances.sort_by_key(|&d| d.0);

            for (_, dist) in distances.iter() {

                print!("\t{}", dist);
            }
            println!();
        }
        // for (idx, item) in as_list.iter().enumerate() {
        //     if idx % self.len() == 0 {
        //         print!("\n{}: ", item.0.0);
        //     }
        //     print!("{:?} ", item.1);
        // }
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