use std::collections::BTreeMap;

use crate::tools::load_input;
use crate::Solution;

pub(crate) struct Day11;

impl Solution for Day11 {
    fn part_1() -> String {
        let data = load_input(11);
        let mut stones: Vec<u128> = data
            .split(" ")
            .map(|n| n.parse().expect("Couldn't parse num"))
            .collect();

        for _ in 0..25 {
            let mut new_stones: Vec<u128> = Vec::new();

            for stone in stones.iter() {
                let d = num_digits(*stone);
                if *stone == 0 {
                    new_stones.push(1);
                } else if d % 2 == 0 {
                    new_stones.push(stone / (10_u128.pow(d / 2)));
                    new_stones.push(stone % (10_u128.pow(d / 2)));
                } else {
                    new_stones.push(*stone * 2024_u128);
                }
            }
            stones = new_stones;
        }

        stones.iter().count().to_string()
    }

    fn part_2() -> String {
        let data = load_input(11);
        let mut stones: BTreeMap<u128, usize> = BTreeMap::new();

        for stone in data
            .split(" ")
            .map(|n| n.parse::<u128>().expect("Couldn't parse num"))
        {
            *stones.entry(stone).or_default() += 1;
        }

        for _ in 0..75 {
            let mut new_stones: BTreeMap<u128, usize> = BTreeMap::new();
            for (stone, n) in stones.iter() {
                let d = num_digits(*stone);
                if *stone == 0 {
                    *new_stones.entry(1).or_default() += n
                } else if d % 2 == 0 {
                    *new_stones.entry(stone / (10_u128.pow(d / 2))).or_default() += n;
                    *new_stones.entry(stone % (10_u128.pow(d / 2))).or_default() += n;
                } else {
                    *new_stones.entry(*stone * 2024_u128).or_default() += n
                }
            }

            stones = new_stones
        }

        stones.iter().map(|(_, n)| n).sum::<usize>().to_string()
    }
}

fn num_digits(n: u128) -> u32 {
    (n as f64 + 0.1).log10().ceil() as u32
}
