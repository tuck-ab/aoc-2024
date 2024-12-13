use std::cmp::{max, min};
use std::collections::{BTreeMap, BTreeSet};

use crate::tools::{load_input, Coord, Dir4, Vec2D, DIR4S};
use crate::Solution;

pub(crate) struct Day12;

impl Solution for Day12 {
    fn part_1() -> String {
        let data = load_input(12);
        let grid = get_grid(data);

        let mut seen: BTreeSet<Coord> = BTreeSet::new();
        let mut total = 0;

        for row in 0..grid.rows() {
            for col in 0..grid.cols() {
                let coord = Coord::from_usize(row, col);

                if seen.contains(&coord) {
                    continue;
                }

                let plot = grid.get(&coord).unwrap();
                let mut area = 0;
                let mut perm = 0;
                let mut to_search: BTreeSet<Coord> = BTreeSet::new();
                to_search.insert(coord.clone());

                while !to_search.is_empty() {
                    let current = to_search.pop_first().unwrap();
                    seen.insert(current);
                    area += 1;
                    for dir in DIR4S {
                        let new_coord = &dir.step(current);
                        match grid.get(&new_coord).map(|c| *c == *plot) {
                            Some(true) => {
                                (!seen.contains(new_coord))
                                    .then(|| to_search.insert(new_coord.clone()));
                            }
                            _ => perm += 1,
                        }
                    }
                }

                total += perm * area;
            }
        }

        total.to_string()
    }

    fn part_2() -> String {
        let data = load_input(12);
        let grid = get_grid(data);

        let mut seen: BTreeSet<Coord> = BTreeSet::new();
        let mut total = 0;

        for row in 0..grid.rows() {
            for col in 0..grid.cols() {
                let coord = Coord::from_usize(row, col);

                if seen.contains(&coord) {
                    continue;
                }

                let plot = grid.get(&coord).unwrap();
                let mut area = 0;
                let mut to_search: BTreeSet<Coord> = BTreeSet::new();
                let mut shape: BTreeSet<Coord> = BTreeSet::new();
                to_search.insert(coord.clone());
                let mut furthest: BTreeMap<Dir4, i32> = BTreeMap::new();

                while !to_search.is_empty() {
                    let current = to_search.pop_first().unwrap();
                    seen.insert(current);
                    shape.insert(current);
                    furthest
                        .entry(Dir4::Up)
                        .and_modify(|v| *v = min(*v, current.row))
                        .or_insert(current.row);
                    furthest
                        .entry(Dir4::Down)
                        .and_modify(|v| *v = max(*v, current.row))
                        .or_insert(current.row);
                    furthest
                        .entry(Dir4::Left)
                        .and_modify(|v| *v = min(*v, current.col))
                        .or_insert(current.col);
                    furthest
                        .entry(Dir4::Right)
                        .and_modify(|v| *v = max(*v, current.col))
                        .or_insert(current.col);
                    area += 1;
                    for dir in DIR4S {
                        let new_coord = &dir.step(current);
                        match grid.get(&new_coord).map(|c| *c == *plot) {
                            Some(true) => {
                                (!seen.contains(new_coord))
                                    .then(|| to_search.insert(new_coord.clone()));
                            }
                            _ => {}
                        }
                    }
                }

                let mut edges = 0;

                // Run vertically
                for row in *furthest.get(&Dir4::Up).unwrap()-1..*furthest.get(&Dir4::Down).unwrap()+1 {
                    let mut block = 0;
                    for col in *furthest.get(&Dir4::Left).unwrap()..*furthest.get(&Dir4::Right).unwrap()+1 {
                        let c1 = shape.contains(&Coord::new(row, col)).then(|| 1).unwrap_or(0);
                        let c2 = shape.contains(&Coord::new(row+1, col)).then(|| 2).unwrap_or(0);

                        if block == 0 && ((c1 != 0) ^ (c2 != 0)) {
                            block = c1 | c2;
                            edges += 1;
                        } else if block != 0 && ((c1 != 0) == (c2 != 0)) {
                            block = 0;
                        } else if block != 0 && (block & (c1 | c2)) == 0 {
                            block = c1 | c2;
                            edges += 1;
                        }
                    }
                }

                // Run horizontally
                for col in *furthest.get(&Dir4::Left).unwrap()-1..*furthest.get(&Dir4::Right).unwrap()+1 {
                    let mut block = 0;
                    for row in *furthest.get(&Dir4::Up).unwrap()..*furthest.get(&Dir4::Down).unwrap()+1 {
                        let c1 = shape.contains(&Coord::new(row, col)).then(|| 1).unwrap_or(0);
                        let c2 = shape.contains(&Coord::new(row, col+1)).then(|| 2).unwrap_or(0);

                        if block == 0 && ((c1 != 0) ^ (c2 != 0)) {
                            block = c1 | c2;
                            edges += 1;
                        } else if block != 0 && ((c1 != 0) == (c2 != 0)) {
                            block = 0;
                        } else if block != 0 && (block & (c1 | c2)) == 0 {
                            block = c1 | c2;
                            edges += 1;
                        }
                        
                    }
                }

                total += edges * area;
            }
        }
       
        total.to_string()
    }
}

fn get_grid(data: String) -> Vec2D<char> {
    let cols = data.lines().next().unwrap().len();
    let rows = data.lines().count();

    let mut grid: Vec2D<char> = Vec2D::new(rows, cols);

    for (row, line) in data.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            let coord = Coord::from_usize(row, col);
            grid.set(&coord, ch);
        }
    }

    grid
}
