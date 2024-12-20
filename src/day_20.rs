use std::collections::BTreeMap;

use crate::tools::{load_input, Coord, Vec2D, DIR4S};
use crate::Solution;

pub(crate) struct Day20;

impl Solution for Day20 {
    fn part_1() -> String {
        let data = load_input(20);
        let (grid, start_pos, end_pos) = get_grid(data);

        let mut path_distances: BTreeMap<Coord, i32> = BTreeMap::new();

        let mut maybe_current: Option<Coord> = None;
        for dir in DIR4S {
            let next = dir.step(start_pos.clone());
            match grid.get(&next) {
                Some(Item::Space) => maybe_current = Some(next),
                _ => {}
            }
        }
        let mut current = maybe_current.unwrap();
        let mut steps = 1;
        path_distances.insert(start_pos, 0);
        path_distances.insert(current, steps);

        while current != end_pos {
            let mut maybe_next: Option<Coord> = None;
            for dir in DIR4S {
                let next = dir.step(current.clone());
                if !path_distances.contains_key(&next) {
                    match grid.get(&next) {
                        Some(Item::Space) => maybe_next = Some(next),
                        _ => {}
                    }
                }
            }
            current = maybe_next.unwrap();
            steps += 1;
            path_distances.insert(current, steps);
        }

        let mut total = 0;

        for (point, steps) in path_distances.iter() {
            for dir in DIR4S {
                let new_point = dir.step(point.clone());
                if grid
                    .get(&new_point)
                    .map(|i| *i == Item::Wall)
                    .unwrap_or(false)
                {
                    for dir2 in DIR4S {
                        let second_point = dir2.step(new_point.clone());
                        if second_point == *point {
                            continue;
                        }
                        match grid.get(&second_point) {
                            Some(Item::Space) => {
                                if path_distances.get(&second_point).unwrap() - *steps > 100 {
                                    total += 1;
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        total.to_string()
    }

    fn part_2() -> String {
        let data = load_input(20);
        let (grid, start_pos, end_pos) = get_grid(data);

        let mut path_distances: BTreeMap<Coord, i32> = BTreeMap::new();

        let mut maybe_current: Option<Coord> = None;
        for dir in DIR4S {
            let next = dir.step(start_pos.clone());
            match grid.get(&next) {
                Some(Item::Space) => maybe_current = Some(next),
                _ => {}
            }
        }
        let mut current = maybe_current.unwrap();
        let mut steps = 1;
        path_distances.insert(start_pos, 0);
        path_distances.insert(current, steps);

        while current != end_pos {
            let mut maybe_next: Option<Coord> = None;
            for dir in DIR4S {
                let next = dir.step(current.clone());
                if !path_distances.contains_key(&next) {
                    match grid.get(&next) {
                        Some(Item::Space) => maybe_next = Some(next),
                        _ => {}
                    }
                }
            }
            current = maybe_next.unwrap();
            steps += 1;
            path_distances.insert(current, steps);
        }

        let mut total = 0;
        for (point, steps) in path_distances.iter() {
            for d_row in -20_i32..21 {
                for d_col in d_row.abs() - 20..21 - d_row.abs() {
                    let new_point = point.clone().apply(d_row, d_col);
                    match path_distances.get(&new_point) {
                        Some(other_steps) => {
                            if other_steps - (steps + new_point.l1_dist(point)) >= 100 {
                                total += 1;
                            }
                        }
                        None => {}
                    }
                }
            }
        }

        total.to_string()
    }
}

#[derive(Default, PartialEq, Eq)]
enum Item {
    Wall,
    #[default]
    Space,
}

impl Item {
    fn from_char(c: char) -> Self {
        match c {
            '#' | ';' => Self::Wall,
            '.' | 'S' | 'E' => Self::Space,
            _ => panic!("Unkown symbol {}", c),
        }
    }
}

fn get_grid(data: String) -> (Vec2D<Item>, Coord, Coord) {
    let cols = data.lines().next().unwrap().len();
    let rows = data.lines().count();
    let mut grid: Vec2D<Item> = Vec2D::new(rows, cols);
    let mut start_pos: Coord = Coord::new(0, 0);
    let mut end_pos: Coord = Coord::new(0, 0);

    for (row, line) in data.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            let coord = Coord::from_usize(row, col);
            grid.set(&coord, Item::from_char(c));
            if c == 'S' {
                start_pos = coord;
            }
            if c == 'E' {
                end_pos = coord;
            }
        }
    }

    (grid, start_pos, end_pos)
}
