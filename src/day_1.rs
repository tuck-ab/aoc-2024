use std::collections::HashMap;

use crate::tools::load_input;
use crate::Solution;

pub(crate) struct Day1;

impl Solution for Day1 {
    fn part_1() {
        let input = load_input(1);
        let (mut left_side, mut right_side) = get_numbers(input);

        left_side.sort();
        right_side.sort();

        let mut total = 0;

        for (left, right) in left_side.iter().zip(right_side.iter()) {
            total += (left - right).abs();
        }

        println!("{}", total);
    }

    fn part_2() {
        let input = load_input(1);
        let (left_side, right_side) = get_numbers(input);

        let counts = right_side
            .iter()
            .fold(HashMap::<i32, i32>::new(), |mut m, n| {
                *m.entry(*n).or_default() += 1;
                m
            });

        let mut total = 0;

        for num in left_side {
            total += num * counts.get(&num).unwrap_or(&0);
        }

        println!("{}", total);
    }
}

fn get_numbers(input: String) -> (Vec<i32>, Vec<i32>) {
    let mut left_side = Vec::<i32>::new();
    let mut right_side = Vec::<i32>::new();

    for line in input.lines() {
        let numbers: Vec<i32> = line
            .split("   ")
            .map(|n| n.parse::<i32>().expect("Couldn't parse num"))
            .collect();

        for (v, num) in vec![&mut left_side, &mut right_side]
            .iter_mut()
            .zip(numbers)
        {
            v.push(num)
        }
    }

    (left_side, right_side)
}
