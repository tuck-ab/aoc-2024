use itertools::{repeat_n, Itertools};

use crate::tools::load_input;
use crate::Solution;

pub(crate) struct Day7;

fn get_op(op: usize) -> fn((i128, i128)) -> i128 {
    match op {
        0 => |(a, b)| a + b,
        1 => |(a, b)| a * b,
        2 => |(a, b)| a * 10_i128.pow((b as f64 + 0.1).log10().ceil() as u32) + b,
        _ => panic!("Op {} not found", op)
    }
}

impl Solution for Day7 {
    fn part_1() -> String {
        let data = load_input(7);

        let mut total = 0;
        'outer: for (t, s) in data.lines().map(|l| l.split(": ").collect_tuple().unwrap()) {
            let target: i128 = t.parse().expect("Couldn't parse number");
            let vals: Vec<i128> = s
                .split(" ")
                .map(|l| l.parse().expect("Couldn't parse nubmer"))
                .collect();

            for ops in repeat_n(0_usize..2_usize, vals.len() - 1).multi_cartesian_product() {
                let mut calc = vals[0];
                for (val, op) in vals[1..].iter().zip(ops.iter()) {
                    calc = get_op(*op)((calc, *val));
                }

                if calc == target {
                    total += calc;
                    continue 'outer;
                }
            }
        }

        total.to_string()
    }

    fn part_2() -> String {
        let data = load_input(7);

        let mut total = 0;
        'outer: for (t, s) in data.lines().map(|l| l.split(": ").collect_tuple().unwrap()) {
            let target: i128 = t.parse().expect("Couldn't parse number");
            let vals: Vec<i128> = s
                .split(" ")
                .map(|l| l.parse().expect("Couldn't parse nubmer"))
                .collect();

            'perm: for ops in repeat_n(0_usize..3_usize, vals.len() - 1).multi_cartesian_product() {
                let mut calc = vals[0];
                for (val, op) in vals[1..].iter().zip(ops.iter()) {
                    calc = get_op(*op)((calc, *val));
                    if calc > target {
                        continue 'perm;
                    }
                }

                if calc == target {
                    total += calc;
                    continue 'outer;
                }
            }
        }

        total.to_string()
    }
}
