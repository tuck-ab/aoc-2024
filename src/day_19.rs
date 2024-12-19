use std::collections::BTreeMap;

use crate::tools::load_input;
use crate::Solution;

pub(crate) struct Day19;

type TowelTree = BTreeMap<usize, Vec<String>>;

impl Solution for Day19 {
    fn part_1() -> String {
        let data = load_input(19);
        let (towel_tree, patterns) = parse_data(data);

        patterns
            .iter()
            .filter(|p| get_pattern(p.as_str(), &towel_tree))
            .count()
            .to_string()
    }

    fn part_2() -> String {
        let data = load_input(19);
        let (towel_tree, patterns) = parse_data(data);

        patterns
            .iter()
            .filter(|p| get_pattern(p.as_str(), &towel_tree))
            .map(|p| get_pattern_count(p.as_str(), &towel_tree, &mut BTreeMap::new()))
            .sum::<usize>()
            .to_string()
    }
}

fn get_pattern(pattern: &str, towel_tree: &TowelTree) -> bool {
    if pattern == "" {
        return true;
    }

    for (towel_length, towels) in towel_tree.iter().rev() {
        if pattern.len() < *towel_length {
            continue;
        }

        for towel in towels.iter() {
            if *towel.as_str() == pattern[..*towel_length] {
                if get_pattern(&pattern[*towel_length..], towel_tree) {
                    return true;
                }
            }
        }
    }

    false
}

fn get_pattern_count<'a>(
    pattern: &'a str,
    towel_tree: &TowelTree,
    cache: &mut BTreeMap<&'a str, usize>,
) -> usize {
    if pattern == "" {
        return 1;
    }

    let mut count = 0;

    for (towel_length, towels) in towel_tree.iter().rev() {
        if pattern.len() < *towel_length {
            continue;
        }

        for towel in towels.iter() {
            if *towel.as_str() == pattern[..*towel_length] {
                let to_check = &pattern[*towel_length..];
                count += match cache.get(to_check) {
                    Some(n) => *n,
                    None => get_pattern_count(to_check, towel_tree, cache),
                }
            }
        }
    }

    cache.insert(pattern, count);

    count
}

fn parse_data(data: String) -> (TowelTree, Vec<String>) {
    let parts: Vec<&str> = data.split("\n\n").collect();

    let towels: Vec<String> = parts[0].split(", ").map(|s| s.to_string()).collect();
    let patterns: Vec<String> = parts[1].lines().map(|s| s.to_string()).collect();

    let mut towels_tree: TowelTree = BTreeMap::new();
    for towel in towels {
        towels_tree.entry(towel.len()).or_default().push(towel)
    }

    (towels_tree, patterns)
}
