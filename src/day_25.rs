use crate::tools::load_input;
use crate::Solution;

pub(crate) struct Day25;

impl Solution for Day25 {
    fn part_1() -> String {
        let data = load_input(25);
        let parsed = parse_data(data);

        let mut keys: Vec<[usize; 5]> = Vec::new();
        let mut locks: Vec<[usize; 5]> = Vec::new();

        for item in parsed {
            if item.is_key {
                keys.push(item.data);
            } else {
                locks.push(item.data);
            }
        }

        let mut overlap = 0;

        for key in keys.iter() {
            'lock_loop: for lock in locks.iter() {
                for (ki, li) in key.iter().zip(lock.iter()) {
                    if ki + li > 7 {
                        continue 'lock_loop;
                    }
                }
                overlap += 1;
            }
        }

        overlap.to_string()
    }

    fn part_2() -> String {
        let _data = load_input(25);
        "We're done!!".to_string()
    }
}

fn parse_data(data: String) -> Vec<Item> {
    data.split("\n\n").map(|s| Item::from_data(s)).collect()
}

struct Item {
    is_key: bool,
    data: [usize; 5],
}

impl Item {
    fn from_data(inp_data: &str) -> Self {
        let is_key = inp_data.lines().next().unwrap().contains(".");
        let mut data = [0; 5];
        if is_key {
            for line in inp_data.lines() {
                for (i, c) in line.chars().enumerate() {
                    if c == '#' {
                        data[i] += 1
                    }
                }
            }
        } else {
            for line in inp_data.lines() {
                for (i, c) in line.chars().enumerate() {
                    if c == '#' {
                        data[i] += 1
                    }
                }
            }
        }

        Self { is_key, data }
    }
}
