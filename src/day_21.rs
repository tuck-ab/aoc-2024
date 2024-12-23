use std::collections::{BTreeMap, BTreeSet};
use std::cmp::min;

use itertools::Itertools;
use regex::Regex;

use crate::tools::{load_input, Coord, Dir4};
use crate::Solution;

pub(crate) struct Day21;

impl Solution for Day21 {
    fn part_1() -> String {
        let data = load_input(21);
        let mut total = 0;
        let mut cache: BTreeMap<(DirPad, DirPad, u64), u64> = BTreeMap::new();

        for code in data.lines() {
            let re = Regex::new(r"^([0-9]+)A$").unwrap();
            let numeric: u64 = re.captures_iter(&code).next().unwrap().extract::<1>().1[0]
                .parse()
                .expect("Could not parse num");

            let mut start_point = NumPad::Accept;
            let robot_steps = 2;
            let mut instructions = 0;

            for c in code.chars() {
                let target = NumPad::from_char(c);
                let initial_options = get_first_posibilites(
                    &start_point.get_coord(),
                    &target.get_coord(),
                    &Coord::new(3, 0),
                );
                let mut min_score: Option<u64> = None;
                for option in initial_options {
                    let mut val = 0;
                    for ds in option.as_slice().windows(2) {
                        val += get_length(&ds[0], &ds[1], robot_steps, &mut cache)
                    }
                    min_score = match min_score {
                        Some(n) => Some(min(n, val)),
                        None => Some(val)
                    }
                }
                instructions += min_score.unwrap();
                start_point = target;
            }
            total += numeric * instructions as u64;
        }
        total.to_string()
    }

    fn part_2() -> String {
        let data = load_input(21);
        let mut total = 0;
        let mut cache: BTreeMap<(DirPad, DirPad, u64), u64> = BTreeMap::new();

        for code in data.lines() {
            let re = Regex::new(r"^([0-9]+)A$").unwrap();
            let numeric: u64 = re.captures_iter(&code).next().unwrap().extract::<1>().1[0]
                .parse()
                .expect("Could not parse num");

            let mut start_point = NumPad::Accept;
            let robot_steps = 25;
            let mut instructions = 0;

            for c in code.chars() {
                let target = NumPad::from_char(c);
                let initial_options = get_first_posibilites(
                    &start_point.get_coord(),
                    &target.get_coord(),
                    &Coord::new(3, 0),
                );
                let mut min_score: Option<u64> = None;
                for option in initial_options {
                    let mut val = 0;
                    for ds in option.as_slice().windows(2) {
                        val += get_length(&ds[0], &ds[1], robot_steps, &mut cache)
                    }
                    min_score = match min_score {
                        Some(n) => Some(min(n, val)),
                        None => Some(val)
                    }
                }
                instructions += min_score.unwrap();
                start_point = target;
            }
            total += numeric * instructions as u64;
        }
        total.to_string()
    }
}

fn get_first_posibilites(from: &Coord, to: &Coord, bad: &Coord) -> BTreeSet<Vec<DirPad>> {
    let instructions = instructions_from_dr_dc(from.dr_dc(to));
    let mut options: BTreeMap<u64, BTreeSet<Vec<DirPad>>> = BTreeMap::new();
    let num_instructions = instructions.len();

    'outer: for mut perm in instructions.into_iter().permutations(num_instructions) {
        let mut current = from.clone();
        for dir in perm.iter() {
            current = dir.get_dir().step(current);
            if current == *bad {
                continue 'outer;
            }
        }
        perm.insert(0, DirPad::Accept);
        perm.push(DirPad::Accept);
        options.entry(perm.len() as u64).or_default().insert(perm);
    }

    options.first_key_value().unwrap().1.clone()
}

fn get_length(from: &DirPad, to: &DirPad, level: u64, cache: &mut BTreeMap<(DirPad, DirPad, u64), u64>) -> u64 {
    if level == 0 {
        return 1
    }

    let key = (*from, *to, level);
    if cache.contains_key(&key) {
        return cache.get(&key).unwrap().clone();
    }

    let instructions = instructions_from_dr_dc(from.get_coord().dr_dc(&to.get_coord()));
    let num_instructions = instructions.len();
    let mut total: Option<u64> = None;

    'outer: for mut perm in instructions.into_iter().permutations(num_instructions) {
        let mut current = from.get_coord();
        for dir in perm.iter() {
            current = dir.get_dir().step(current);
            if current == Coord::new(0, 0) {
                continue 'outer;
            }
        }
        perm.insert(0, DirPad::Accept);
        perm.push(DirPad::Accept);

        let mut total_length = 0;
        for ds in perm.as_slice().windows(2) {
            total_length += get_length(&ds[0], &ds[1], level-1, cache)
        }
        total = match total {
            Some(n) => Some(min(n, total_length)),
            None => Some(total_length)
        }
    }

    cache.insert(key, total.unwrap());
    total.unwrap()
}

fn instructions_from_dr_dc(dr_dc: (i32, i32)) -> Vec<DirPad> {
    let mut instructions: Vec<DirPad> = Vec::new();

    if dr_dc.1 > 0 {
        for _ in 0..dr_dc.1.abs() {
            instructions.push(DirPad::Right)
        }
    } else {
        for _ in 0..dr_dc.1.abs() {
            instructions.push(DirPad::Left)
        }
    }

    if dr_dc.0 > 0 {
        for _ in 0..dr_dc.0.abs() {
            instructions.push(DirPad::Down)
        }
    } else {
        for _ in 0..dr_dc.0.abs() {
            instructions.push(DirPad::Up)
        }
    }

    instructions
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum NumPad {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,
    Accept,
}

impl NumPad {
    fn get_coord(&self) -> Coord {
        match *self {
            Self::One => Coord::new(2, 0),
            Self::Two => Coord::new(2, 1),
            Self::Three => Coord::new(2, 2),
            Self::Four => Coord::new(1, 0),
            Self::Five => Coord::new(1, 1),
            Self::Six => Coord::new(1, 2),
            Self::Seven => Coord::new(0, 0),
            Self::Eight => Coord::new(0, 1),
            Self::Nine => Coord::new(0, 2),
            Self::Zero => Coord::new(3, 1),
            Self::Accept => Coord::new(3, 2),
        }
    }

    fn from_char(c: char) -> Self {
        match c {
            '1' => Self::One,
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            '0' => Self::Zero,
            'A' => Self::Accept,
            _ => panic!("Unexpected character '{}'", c),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum DirPad {
    Up,
    Down,
    Left,
    Right,
    Accept,
}

impl std::fmt::Debug for DirPad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Up => write!(f, "^")?,
            Self::Down => write!(f, "v")?,
            Self::Left => write!(f, "<")?,
            Self::Right => write!(f, ">")?,
            Self::Accept => write!(f, "A")?,
        }

        Ok(())
    }
}

impl DirPad {
    fn get_coord(&self) -> Coord {
        match *self {
            Self::Up => Coord::new(0, 1),
            Self::Down => Coord::new(1, 1),
            Self::Left => Coord::new(1, 0),
            Self::Right => Coord::new(1, 2),
            Self::Accept => Coord::new(0, 2),
        }
    }

    fn get_dir(&self) -> Dir4 {
        match *self {
            Self::Up => Dir4::Up,
            Self::Down => Dir4::Down,
            Self::Left => Dir4::Left,
            Self::Right => Dir4::Right,
            Self::Accept => panic!("Accept has no dir"),
        }
    }
}
