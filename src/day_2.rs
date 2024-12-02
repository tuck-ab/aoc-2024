use crate::tools::load_input;
use crate::Solution;

pub(crate) struct Day2;

impl Solution for Day2 {
    fn part_1() {
        let data = load_input(2);

        let mut total = 0;

        for line in data.lines() {
            let mut ops: u8 = 0;
            let values: Vec<i32> = line
                .split(" ")
                .map(|v| v.parse::<i32>().expect("Could not parse number"))
                .collect();
            for (v1, v2) in values.iter().zip(values[1..].iter()) {
                if v1 < v2 {
                    ops |= 1
                } else if v1 > v2 {
                    ops |= 2
                } else {
                    ops |= 4
                }

                if (v1 - v2).abs() > 3 {
                    ops |= 4
                }
            }

            if ops == 1 || ops == 2 {
                total += 1
            }
        }

        println!("{}", total);
    }

    fn part_2() {
        let data = load_input(2);

        let mut total = 0;

        for line in data.lines() {
            let res = check_safe(line, None);
            if res.is_none() {
                total += 1;
                continue;
            }

            let error_index = res.unwrap();
            

            if check_safe(line, Some(error_index)).is_none()
                || check_safe(line, Some(error_index + 1)).is_none()
            {
                total += 1;
            } else {
                println!("{}", line);
                println!("{}", error_index);
                break;
            }
            
        }

        println!("{}", total)
    }
}

fn check_safe(line: &str, remove_index: Option<usize>) -> Option<usize> {
    let mut values: Vec<i32> = line
        .split(" ")
        .map(|v| v.parse::<i32>().expect("Could not parse number"))
        .collect();

    match remove_index {
        Some(index) => {
            values.remove(index);
        }
        _ => (),
    };

    let mut flag = 0;
    for (i, (v1, v2)) in values.iter().zip(values[1..].iter()).enumerate() {
        if v1 < v2 {
            flag |= 1
        } else if v1 > v2 {
            flag |= 2
        } else {
            flag |= 4
        }

        if (v1 - v2).abs() > 3 {
            flag |= 4
        }

        if !(flag == 1 || flag == 2) {
            return Some(i);
        }
    }

    None
}
