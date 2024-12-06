use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::collections::{HashMap, HashSet};
use std::iter::Iterator;
use std::rc::Rc;

use itertools::Itertools;

use crate::tools::load_input;
use crate::Solution;

type RulesMap = HashMap<usize, HashSet<usize>>;

pub(crate) struct Day5;

impl Solution for Day5 {
    fn part_1() -> String {
        let data = load_input(5);
        parse_input(data)
            .into_iter()
            .filter(|c| c.is_sorted())
            .map(|c| c[&c.len() / 2].num)
            .sum::<usize>()
            .to_string()
    }

    fn part_2() -> String {
        let data = load_input(5);
        parse_input(data)
            .into_iter()
            .filter(|c| !c.is_sorted())
            .map(|c| c.into_iter().sorted().collect::<Vec<Num>>())
            .map(|c| c[&c.len() / 2].num)
            .sum::<usize>()
            .to_string()
    }
}

fn parse_input(data: String) -> Vec<Vec<Num>> {
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
        Rc::get_mut(&mut afters)
            .unwrap()
            .entry(before)
            .or_default()
            .insert(after);
    }

    parts[1]
        .lines()
        .map(|l| {
            l.split(',')
                .map(|c| Num {
                    num: c.parse::<usize>().expect("Couldn't parse number"),
                    map: afters.clone(),
                })
                .collect::<Vec<Num>>()
        })
        .collect()
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
