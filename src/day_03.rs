use regex::Regex;

use crate::tools::load_input;
use crate::Solution;

pub(crate) struct Day3;

impl Solution for Day3 {
    fn part_1() -> String {
        let data = load_input(3);
        let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
        re.captures_iter(&data)
            .map(|c| c.extract())
            .map(|(_, [n1, n2])| n1.parse::<i32>().unwrap() * n2.parse::<i32>().unwrap())
            .sum::<i32>()
            .to_string()
    }

    fn part_2() -> String {
        let data = load_input(3);
        let mul_re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
        let do_re = Regex::new(r"do\(\)").unwrap();
        let dont_re = Regex::new(r"don't\(\)").unwrap();

        let do_locs: Vec<usize> = do_re
            .find_iter(&data)
            .map(|c| c.range().nth(0).unwrap())
            .collect();
        let dont_locs: Vec<usize> = dont_re
            .find_iter(&data)
            .map(|c| c.range().nth(0).unwrap())
            .collect();

        let mut do_index = 0;
        let mut do_string: String = "".to_string();

        loop {
            let dont_index = match dont_locs.iter().filter(|x| **x > do_index).nth(0) {
                Some(x) => *x,
                None => break,
            };
            do_string = do_string + &data[do_index..dont_index];
            do_index = match do_locs.iter().filter(|x| **x > dont_index).nth(0) {
                Some(x) => *x,
                None => break,
            };
        }

        mul_re
            .captures_iter(&do_string)
            .map(|c| c.extract())
            .map(|(_, [n1, n2])| n1.parse::<i32>().unwrap() * n2.parse::<i32>().unwrap())
            .sum::<i32>()
            .to_string()
    }
}

impl Day3 {
    #[allow(dead_code)]
    pub fn charlotte_sol() {
        let data = load_input(3);
        let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)|(d)(o)\(\)|(d)(o)n't\(\)").unwrap();

        let mut flag = true;

        let total: i32 = re
            .captures_iter(&data)
            .map(|c| c.extract())
            .map(|(m, [n1, n2])| match m {
                "do()" => {
                    flag = true;
                    0
                }
                "don't()" => {
                    flag = false;
                    0
                }
                _ => flag
                    .then(|| n1.parse::<i32>().unwrap() * n2.parse::<i32>().unwrap())
                    .unwrap_or(0),
            })
            .sum();

        println!("{}", total)
    }
}
