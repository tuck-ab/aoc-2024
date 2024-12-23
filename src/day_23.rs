use std::collections::{BTreeMap, BTreeSet};

use itertools::Itertools;
use regex::Regex;

use crate::tools::load_input;
use crate::Solution;

pub(crate) struct Day23;

impl Solution for Day23 {
    fn part_1() -> String {
        let data = load_input(23);
        let graph = Graph::from_data(data);
        let mut trongles: BTreeSet<BTreeSet<u16>> = BTreeSet::new();

        for node in graph.nodes.iter() {
            if is_t(node) {
                for other in graph.edges.get(node).unwrap() {
                    for third in graph.edges.get(other).unwrap() {
                        if graph.edges.get(third).unwrap().contains(node) {
                            trongles.insert(BTreeSet::from([*node, *other, *third]));
                        }
                    }
                }
            }
        }
        trongles.len().to_string()
    }

    fn part_2() -> String {
        let data = load_input(23);

        let graph = Graph::from_data(data);
        let mut result: BTreeSet<u16> = BTreeSet::new();

        let mut n = 3;

        loop {
            match graph.get_sub_graph(BTreeSet::new(), graph.nodes.clone(), BTreeSet::new(), n) {
                Some(r) => result = r,
                None => break,
            }
            n += 1;
        }

        result.iter().map(|n| to_str(n)).join(",").to_string()
    }
}

#[derive(Debug)]
struct Graph {
    nodes: BTreeSet<u16>,
    edges: BTreeMap<u16, BTreeSet<u16>>,
}

impl Graph {
    fn from_data(data: String) -> Self {
        let mut nodes: BTreeSet<u16> = BTreeSet::new();
        let mut edges: BTreeMap<u16, BTreeSet<u16>> = BTreeMap::new();

        let re = Regex::new(r"^([a-z]{2})-([a-z]{2})$").unwrap();

        for line in data.lines() {
            let ns = re
                .captures_iter(line)
                .exactly_one()
                .unwrap()
                .extract::<2>()
                .1
                .map(|c| to_u16(c));

            nodes.insert(ns[0]);
            nodes.insert(ns[1]);
            edges.entry(ns[0]).or_default().insert(ns[1]);
            edges.entry(ns[1]).or_default().insert(ns[0]);
        }

        Self { nodes, edges }
    }

    fn get_sub_graph(
        &self,
        partial: BTreeSet<u16>,
        candidates: BTreeSet<u16>,
        mut ignore: BTreeSet<u16>,
        n: usize,
    ) -> Option<BTreeSet<u16>> {
        if n == 0 {
            return Some(partial);
        }

        for c in candidates.iter() {
            if ignore.contains(c) {
                continue;
            }

            let mut new = partial.clone();
            new.insert(*c);
            let new_can: BTreeSet<u16> = candidates
                .intersection(self.edges.get(c).unwrap())
                .cloned()
                .collect();
            match self.get_sub_graph(new, new_can, ignore.clone(), n - 1) {
                Some(n) => return Some(n),
                None => {
                    ignore.insert(*c);
                }
            }
        }

        None
    }
}

fn to_u16(node: &str) -> u16 {
    let mut total = 0;
    for c in node.chars() {
        total = total << 8;
        total += c as u16;
    }
    total
}

fn is_t(n: &u16) -> bool {
    (*n & 0b11111111_00000000) == 0b01110100_00000000
}

fn to_str(n: &u16) -> String {
    [
        char::from_u32(((*n & 0b11111111_00000000) >> 8) as u32).unwrap(),
        char::from_u32((*n & 0b11111111) as u32).unwrap(),
    ]
    .iter()
    .collect()
}
