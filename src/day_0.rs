use crate::tools::load_input;
use crate::Solution;

pub(crate) struct Day0;

impl Solution for Day0 {
    fn part_1() -> String {
        let input = load_input(0);
        println!("Day 0, part 1");
        format!("{}", input)
    }

    fn part_2() -> String {
        format!("Day 0, part 2")
    }
}
