use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::tools::{load_input, Coord, Vec2D};
use crate::Solution;

pub(crate) struct Day8;

impl Solution for Day8 {
    fn part_1() -> String {
        let data = load_input(8);
        let (grid, locs) = get_grid(data);
        let mut antinodes: HashSet<Coord> = HashSet::new();

        for (_tower, locs) in locs.iter() {
            for (l1, l2) in locs.iter().combinations(2).map(|lv| (lv[0], lv[1])) {
                let dr = l2.row - l1.row;
                let dc = l2.col - l1.col;
                // l2.0 + dr, l2.1 + dc)
                let new_loc = l2.clone().apply(dr, dc);
                if grid.in_grid(&new_loc) {
                    antinodes.insert(new_loc);
                }
                let new_loc = l1.clone().apply(-dr, -dc);
                if grid.in_grid(&new_loc) {
                    antinodes.insert(new_loc);
                }
            }
        }

        antinodes.iter().count().to_string()
    }

    fn part_2() -> String {
        let data = load_input(8);
        let (grid, locs) = get_grid(data);
        let mut antinodes: HashSet<Coord> = HashSet::new();

        for (_tower, locs) in locs.iter() {
            for (l1, l2) in locs.iter().combinations(2).map(|lv| (lv[0], lv[1])) {
                let dr = l2.row - l1.row;
                let dc = l2.col - l1.col;

                // current loc
                let mut cl = l1.clone();
                while grid.in_grid(&cl) {
                    antinodes.insert(cl.clone());
                    cl = cl.apply(dr, dc);
                }
                let mut cl = l2.clone();
                while grid.in_grid(&cl) {
                    antinodes.insert(cl.clone());
                    cl = cl.apply(-dr, -dc);
                }
            }
        }

        antinodes.iter().count().to_string()
    }
}

fn get_grid(data: String) -> (Vec2D<char>, HashMap<char, Vec<Coord>>) {
    let cols = data.lines().next().unwrap().len();
    let rows = data.lines().count();

    let mut grid: Vec2D<char> = Vec2D::new(rows, cols);
    let mut locs: HashMap<char, Vec<Coord>> = HashMap::new();

    for (row, line) in data.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            let coord = Coord::from_usize(row, col);
            grid.set(&coord, ch);
            if ch != '.' {
                locs.entry(ch).or_default().push(coord)
            }
        }
    }

    (grid, locs)
}
