use itertools::{repeat_n, Itertools};

use crate::tools::load_input;
use crate::Solution;

pub(crate) struct Day7;

impl Solution for Day7 {
    fn part_1() -> String {
        let data = load_input(7);

        let mut total = 0;
        for (t, s) in data.lines().map(|l| l.split(": ").collect_tuple().unwrap()) {
            let target: i128 = t.parse().expect("Couldn't parse number");
            let vals: Vec<i128> = s
                .split(" ")
                .map(|l| l.parse().expect("Couldn't parse nubmer"))
                .collect();
            total += run_line(target, &vals, &vec![Op::Add, Op::Mul]).unwrap_or(0);
        } 

        total.to_string()
    }

    fn part_2() -> String {
        let data = load_input(7);

        let mut total = 0;
        for (t, s) in data.lines().map(|l| l.split(": ").collect_tuple().unwrap()) {
            let target: i128 = t.parse().expect("Couldn't parse number");
            let vals: Vec<i128> = s
                .split(" ")
                .map(|l| l.parse().expect("Couldn't parse nubmer"))
                .collect();
            total += run_line(target, &vals, &vec![Op::Add, Op::Mul, Op::Cat]).unwrap_or(0);
        }

        total.to_string()
    }
}

enum Op {
    Add,
    Mul,
    Cat,
}

impl Op {
    fn calc(&self, a: i128, b: i128) -> i128 {
        match *self {
            Op::Add => a + b,
            Op::Mul => a * b,
            Op::Cat => a * 10_i128.pow((b as f64 + 0.1).log10().ceil() as u32) + b,
        }
    }
}

fn run_line(target: i128, line: &Vec<i128>, ops: &Vec<Op>) -> Option<i128> {
    for ops in repeat_n(ops, line.len() - 1).multi_cartesian_product() {
        let mut calc = line[0];
        for (val, op) in line[1..].iter().zip(ops.iter()) {
            calc = op.calc(calc, *val);
        }

        if calc == target {
            return Some(target);
        }
    }
    None
}
