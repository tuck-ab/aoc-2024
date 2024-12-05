use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::iter::Iterator;

use itertools::Itertools;

use crate::tools::load_input;
use crate::Solution;

type RulesMap = HashMap<usize, HashSet<usize>>;

pub(crate) struct Day5;

impl Solution for Day5 {
    fn part_1() {
        let data = load_input(5);
        let (afters, chains) = parse_input(data);

        let mut total = 0;
        'outer: for chain in chains {
            let mut befores = Vec::<usize>::new();
            for num in chain.iter() {
                for b in befores.iter() {
                    if afters.get(&(*num).num).and_then(|c| Some(c.contains(b))).unwrap_or(false) {
                        continue 'outer;
                    }
                }
                befores.push((*num).num);
            }
            let index = chain.len() / 2;
            total += chain[index].num;
        }

        println!("{}", total)
    }

    fn part_2() {
        let data = load_input(5);
        let (afters, chains) = parse_input(data);

        let unsorted: Vec<Vec<Num>> = chains
            .into_iter()
            .filter(|c| !c.is_sorted())
            .collect();

        let mut total = 0;
        for mut chain in unsorted {
            chain.sort();
            let index = chain.len() / 2;
            total += chain[index].num;
        }

        println!("{}", total)
    }
}

fn parse_input(data: String) -> (Rc<RulesMap>, Vec<Vec<Num>>) {
    let parts: Vec<String> = data.split("\n\n").map(|s| s.to_string()).collect();

    let rules: Vec<(usize, usize)> = parts[0]
        .lines()
        .map(|l| {
            l.split('|')
                .map(|c| c.parse::<usize>().expect("Couldn't parse number"))
                .collect_tuple::<(usize, usize)>()
                .unwrap()
        })
        .collect();

    // Constrains for numbers that must appear after a number
    let mut afters: Rc<RulesMap> = Rc::new(HashMap::new());
    for (before, after) in rules {
        Rc::get_mut(&mut afters).unwrap().entry(before).or_default().insert(after);
    }

    let chains: Vec<Vec<Num>> = parts[1]
        .lines()
        .map(|l| {
            l.split(',')
                .map(|c| Num { num: c.parse::<usize>().expect("Couldn't parse number"), map: afters.clone() })
                .collect::<Vec<Num>>()
        })
        .collect();

    (afters, chains)
}

struct Num {
    num: usize,
    map: Rc<RulesMap>,
}

impl PartialEq for Num {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num
    }
}

impl Eq for Num {}

impl PartialOrd for Num {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (
            self.map
                .get(&self.num)
                .and_then(|m| Some(m.contains(&other.num)))
                .unwrap_or(false),
            other
                .map
                .get(&other.num)
                .and_then(|m| Some(m.contains(&self.num)))
                .unwrap_or(false),
        ) {
            (true, false) => Some(Ordering::Less),
            (false, true) => Some(Ordering::Greater),
            _ => Some(Ordering::Equal),
        }
    }
}

impl Ord for Num {
    fn cmp(&self, other: &Self) -> Ordering {
        match (
            self.map
                .get(&self.num)
                .and_then(|m| Some(m.contains(&other.num)))
                .unwrap_or(false),
            other
                .map
                .get(&other.num)
                .and_then(|m| Some(m.contains(&self.num)))
                .unwrap_or(false),
        ) {
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            _ => Ordering::Equal,
        }
    }
}
