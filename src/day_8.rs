use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::tools::{load_input, Vec2D};
use crate::Solution;

pub(crate) struct Day8;

impl Solution for Day8 {
    fn part_1() -> String {
        let data = load_input(8);
        let (grid, locs) = get_grid(data);
        let mut antinodes: HashSet<(usize, usize)> = HashSet::new();

        for (_tower, locs) in locs.iter() {
            for (l1, l2) in locs.iter().combinations(2).map(|lv| (lv[0], lv[1])) {
                let dr = l2.0 - l1.0;
                let dc = l2.1 - l1.1;

                match grid.as_coord(&(l2.0 + dr, l2.1 + dc)) {
                    Some(loc) => antinodes.insert(loc),
                    None => false
                };
                match grid.as_coord(&(l1.0 - dr, l1.1 - dc)) {
                    Some(loc) => antinodes.insert(loc),
                    None => false
                };
            }
        }

        antinodes.iter().count().to_string()
    }

    fn part_2() -> String {
        let data = load_input(8);
        let (grid, locs) = get_grid(data);
        let mut antinodes: HashSet<(usize, usize)> = HashSet::new();

        for (_tower, locs) in locs.iter() {
            for (l1, l2) in locs.iter().combinations(2).map(|lv| (lv[0], lv[1])) {
                let dr = l2.0 - l1.0;
                let dc = l2.1 - l1.1;

                // current loc
                let mut cl = l1.clone();
                while let Some(loc) = grid.as_coord(&(cl.0 + dr, cl.1 + dc)) {
                    antinodes.insert(loc);
                    cl = (loc.0 as i32, loc.1 as i32);
                }
                let mut cl = l2.clone();
                while let Some(loc) = grid.as_coord(&(cl.0 - dr, cl.1 - dc)) {
                    antinodes.insert(loc);
                    cl = (loc.0 as i32, loc.1 as i32);
                }
            }
        }

        antinodes.iter().count().to_string()
    }
}

fn get_grid(data: String) -> (Vec2D<char>, HashMap<char, Vec<(i32, i32)>>) {
    let cols = data.lines().next().unwrap().len();
    let rows = data.lines().count();

    let mut grid: Vec2D<char> = Vec2D::new(rows, cols);
    let mut locs: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for (row, line) in data.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            grid.set(row, col, ch);
            if ch != '.' {
                locs.entry(ch).or_default().push((row as i32, col as i32))
            }
        }
    }

    (grid, locs)
}
