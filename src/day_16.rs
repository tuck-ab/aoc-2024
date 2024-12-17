use std::collections::{BTreeMap, BTreeSet};

use crate::tools::{load_input, Coord, Dir4, Vec2D, DIR4S};
use crate::Solution;

pub(crate) struct Day16;

impl Solution for Day16 {
    fn part_1() -> String {
        let data = load_input(16);
        let (grid, start_pos, end_pos) = get_grid(data);

        let mut queue: BTreeMap<u32, Vec<(Dir4, Coord)>> = BTreeMap::new();
        let mut seen: BTreeSet<Coord> = BTreeSet::new();

        queue.entry(0).or_default().push((Dir4::Right, start_pos));

        let path_length;

        loop {
            let key = queue.first_entry().unwrap().key().clone();
            let (dir, pos) = queue.get_mut(&key).unwrap().pop().unwrap();

            if pos == end_pos {
                path_length = key;
                break;
            }

            if queue.get(&key).unwrap().is_empty() {
                let _ = queue.remove(&key);
            }

            if seen.contains(&pos) {
                continue;
            }

            seen.insert(pos.clone());

            for d in DIR4S.iter() {
                let new_coord = d.step(pos.clone());
                if let Some(item) = grid.get(&new_coord) {
                    if *item == Item::Space && !seen.contains(&new_coord) {
                        queue
                            .entry(key + 1 + 1000 * (dir.rotations(&d) as u32 % 2))
                            .or_default()
                            .push((d.clone(), new_coord))
                    }
                }
            }
        }

        path_length.to_string()
    }

    fn part_2() -> String {
        let data = load_input(16);
        let (grid, start_pos, end_pos) = get_grid(data);

        let mut queue: BTreeMap<u32, BTreeMap<(Dir4, Coord), BTreeSet<Coord>>> = BTreeMap::new();
        let mut seen: BTreeSet<(Dir4, Coord)> = BTreeSet::new();

        queue
            .entry(0)
            .or_default()
            .insert((Dir4::Right, start_pos), BTreeSet::from_iter([start_pos]));
        let seats;

        loop {
            let key = queue.first_entry().unwrap().key().clone();

            let ((dir, pos), mut set) = queue.get_mut(&key).unwrap().pop_first().unwrap();

            if pos == end_pos {
                seats = set.len();
                break;
            }

            set.insert(pos.clone());

            if queue.get(&key).unwrap().is_empty() {
                let _ = queue.remove(&key);
            }

            if seen.contains(&(dir, pos)) {
                continue;
            }

            seen.insert((dir.clone(), pos.clone()));

            let new_coord = dir.step(pos.clone());
            if *grid.get(&new_coord).unwrap() == Item::Space && !seen.contains(&(dir, new_coord)) {
                queue
                    .entry(key + 1)
                    .or_default()
                    .entry((dir.clone(), new_coord))
                    .or_default()
                    .append(&mut (set.clone()));
            }

            for d in DIR4S.iter() {
                if !seen.contains(&(d.clone(), pos)) {
                    queue
                        .entry(key + 1000 * (dir.rotations(&d) as u32 % 2))
                        .or_default()
                        .entry((d.clone(), pos))
                        .or_default()
                        .append(&mut (set.clone()));
                }
            }
        }

        (seats + 1).to_string()
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
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
