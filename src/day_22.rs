use std::collections::{BTreeMap, BTreeSet};

use crate::tools::load_input;
use crate::Solution;

pub(crate) struct Day22;

impl Solution for Day22 {
    fn part_1() -> String {
        let data = load_input(22);
        let mut total = 0;
        for line in data.lines() {
            let initial: u64 = line.parse().expect("Could not parse number");
            let mut secret = initial;
            for _ in 0..2000 {
                secret = prune(mix(secret * 64, secret));
                secret = prune(mix(secret / 32, secret));
                secret = prune(mix(secret * 2048, secret));
            }
            total += secret;
        }
        total.to_string()
    }

    fn part_2() -> String {
        let data = load_input(22);
        
        let mut buyers_maps: BTreeMap<(i32, i32, i32, i32), u64> = BTreeMap::new();

        for line in data.lines() {
            let initial: u64 = line.parse().expect("Could not parse number");
            let mut secrets: Vec<u64> = Vec::with_capacity(2001);
            let mut seen: BTreeSet<(i32, i32, i32, i32)> = BTreeSet::new();
            let mut secret = initial;
            secrets.push(secret);

            for _ in 0..2000 {
                secret = prune(mix(secret * 64, secret));
                secret = prune(mix(secret / 32, secret));
                secret = prune(mix(secret * 2048, secret));
                secrets.push(secret);
            }

            let diffs: Vec<i32> = secrets
                .iter()
                .as_slice()
                .windows(2)
                .map(|ds| (ds[1] % 10) as i32 - (ds[0] % 10) as i32)
                .collect();

            for (diffs, value) in diffs.as_slice().windows(4).zip(secrets[4..].iter()) {
                let key = (diffs[0], diffs[1], diffs[2], diffs[3]);
                if !seen.contains(&key) {
                    *buyers_maps.entry(key).or_default() += *value % 10;
                    seen.insert(key);
                }
            }
        }

        buyers_maps.values().max().unwrap().to_string()
    }
}

fn mix(value: u64, secret: u64) -> u64 {
    value ^ secret
}

fn prune(secret: u64) -> u64 {
    secret % 16777216
}
