use std::ops::Not;

use crate::tools::load_input;
use crate::Solution;

pub(crate) struct Day2;

impl Solution for Day2 {
    fn part_1() {
        let data = load_input(2);

        let total = data
            .lines()
            .map(|l| check_safe(l, None))
            .filter(|r| r.is_none())
            .count();

        println!("{}", total);
    }

    fn part_2() {
        let data = load_input(2);

        let total = data
            .lines()
            .map(|l| {
                check_safe(l, None).map(|i| (l, i)).and_then(|(l, i)| {
                    (0..i + 2)
                        .rev()
                        .zip(0..3)
                        .any(|(x, _)| check_safe(l, Some(x)).is_none())
                        .not()
                        .then(|| 0)
                })
            })
            .filter(|x| x.is_none())
            .count();

        println!("{}", total)
    }
}

fn check_safe(line: &str, remove_index: Option<usize>) -> Option<usize> {
    let mut values: Vec<i32> = line
        .split(" ")
        .map(|v| v.parse::<i32>().expect("Could not parse number"))
        .collect();

    remove_index.map(|i| values.remove(i));

    let mut flag = 0;
    for (i, (v1, v2)) in values.iter().zip(values[1..].iter()).enumerate() {
        flag |= 1 << (v1 < v2) as i8;

        if flag == 3 || (v1 - v2).abs() > 3 || v1 == v2 {
            return Some(i);
        }
    }

    None
}
