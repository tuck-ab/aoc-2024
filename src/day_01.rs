use std::collections::HashMap;

use crate::tools::load_input;
use crate::Solution;

pub(crate) struct Day1;

impl Solution for Day1 {
    fn part_1() -> String {
        let input = load_input(1);
        let (mut left_side, mut right_side) = get_numbers(input);

        left_side.sort();
        right_side.sort();

        let total = left_side
            .iter()
            .zip(right_side.iter())
            .map(|(left, right)| (left - right).abs())
            .sum::<i32>();

        total.to_string()
    }

    fn part_2() -> String {
        let input = load_input(1);
        let (left_side, right_side) = get_numbers(input);

        let counts = right_side
            .iter()
            .fold(HashMap::<i32, i32>::new(), |mut m, n| {
                *m.entry(*n).or_default() += 1;
                m
            });

        let total = left_side
            .iter()
            .map(|n| n * counts.get(n).unwrap_or(&0))
            .sum::<i32>();

        total.to_string()
    }
}

fn get_numbers(input: String) -> (Vec<i32>, Vec<i32>) {
    let mut left_side = Vec::<i32>::new();
    let mut right_side = Vec::<i32>::new();

    for line in input.lines() {
        for (n, v) in line
            .split("   ")
            .map(|n| n.parse::<i32>().expect("Couldn't parse num"))
            .zip([&mut left_side, &mut right_side])
        {
            v.push(n);
        }
    }

    (left_side, right_side)
}
