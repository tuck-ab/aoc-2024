use std::collections::BTreeMap;

use crate::tools::{load_input, Coord, Vec2D, DIR4S};
use crate::Solution;

pub(crate) struct Day18;

impl Solution for Day18 {
    fn part_1() -> String {
        let data = load_input(18);
        let mut grid: Vec2D<Item> = Vec2D::new(71, 71);
        let coords: Vec<(i32, i32)> = data
            .lines()
            .map(|l| {
                let parts: Vec<i32> = l
                    .split(",")
                    .map(|n| n.parse::<i32>().expect("Could not parse number"))
                    .collect();
                (parts[0], parts[1])
            })
            .collect();

        for i in 0..1024 {
            let (col, row) = coords[i];
            grid.set(&Coord::new(row, col), Item::Wall)
        }

        let (steps, _) = run_astar(&grid).unwrap();

        steps.to_string()
    }

    fn part_2() -> String {
        let data = load_input(18);
        let mut grid: Vec2D<Item> = Vec2D::new(71, 71);
        let coords: Vec<(i32, i32)> = data
            .lines()
            .map(|l| {
                let parts: Vec<i32> = l
                    .split(",")
                    .map(|n| n.parse::<i32>().expect("Could not parse number"))
                    .collect();
                (parts[0], parts[1])
            })
            .collect();

        for i in 0..1024 {
            let (col, row) = coords[i];
            grid.set(&Coord::new(row, col), Item::Wall)
        }

        let (_, mut path) = run_astar(&grid).unwrap();

        let mut counter = 1024;

        loop {
            let (col, row) = coords[counter];
            let coord = Coord::new(row, col);
            grid.set(&coord, Item::Wall);

            if path.contains(&coord) {
                match run_astar(&grid) {
                    Some((_, new_path)) => path = new_path,
                    None => return format!("{},{}", coord.col, coord.row),
                }
            }

            counter += 1;
        }
    }
}

fn run_astar(grid: &Vec2D<Item>) -> Option<(i32, Vec<Coord>)> {
    let target = Coord::new(70, 70);

    let mut seen: BTreeMap<Coord, Coord> = BTreeMap::new();
    let mut queue: BTreeMap<i32, Vec<(Coord, Coord, i32)>> = BTreeMap::new();

    queue.insert(0, vec![(Coord::new(0, 0), Coord::new(0, 0), 0)]);

    loop {
        if queue.is_empty() {
            return None;
        }

        let key = queue.first_key_value().unwrap().0.clone();
        let pos_vec = queue.get_mut(&key).unwrap();

        let (current, previous, steps) = pos_vec.pop().unwrap();

        seen.insert(current.clone(), previous.clone());

        if current == target {
            let mut backtrack_current = target;
            let mut path: Vec<Coord> = Vec::new();
            while backtrack_current != Coord::new(0, 0) {
                path.push(backtrack_current);
                backtrack_current = *seen.get(&backtrack_current).unwrap();
            }
            return Some((steps, path));
        }

        if pos_vec.is_empty() {
            queue.remove(&key);
        }

        for dir in DIR4S {
            let new_coord = dir.step(current.clone());
            if !seen.contains_key(&new_coord)
                && grid
                    .get(&new_coord)
                    .map(|i| *i != Item::Wall)
                    .unwrap_or(false)
            {
                queue
                    .entry(steps + 1 + new_coord.l1_dist(&target))
                    .or_default()
                    .push((new_coord, current.clone(), steps + 1))
            }
        }
    }
}

#[derive(Default, PartialEq, Eq)]
enum Item {
    Wall,
    #[default]
    Empty,
}
