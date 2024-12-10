use std::collections::HashSet;

use crate::tools::{load_input, Coord, Vec2D, Dir4};
use crate::Solution;

pub(crate) struct Day6;

impl Solution for Day6 {
    fn part_1() -> String {
        let data = load_input(6);
        let (mut grid, mut pos) = get_grid(data);
        let mut dir = 0;

        let dirs = [Dir4::Up, Dir4::Right, Dir4::Down, Dir4::Left];
        grid.set(&pos, 'X');

        loop {
            let next_step = dirs[dir].step(pos);
            match &grid.get(&next_step) {
                Some('#') => dir = (dir + 1) % 4,
                Some(_c) => {
                    grid.set(&next_step, 'X');
                    pos = next_step;
                }
                None => break,
            }
        }

        grid.data().iter().filter(|c| **c == 'X').count().to_string()
    }

    fn part_2() -> String {
        let data = load_input(6);
        let (mut grid, start_pos) = get_grid(data);

        let mut dir = 0;
        let mut pos = start_pos.clone();

        let dirs = [Dir4::Up, Dir4::Right, Dir4::Down, Dir4::Left];

        loop {
            let next_step = dirs[dir].step(pos);
            match &grid.get(&next_step) {
                Some('#') => dir = (dir + 1) % 4,
                Some(_c) => {
                    grid.set(&next_step, 'X');
                    pos = next_step;
                }
                None => break,
            }
        }

        let path_points: Vec<Coord> = grid
            .data()
            .iter()
            .enumerate()
            .filter(|(_, c)| **c == 'X')
            .map(|(i, _)| grid.get_coord(i))
            .collect();

        let mut total = 0;
        for coord in path_points {
            grid.set(&coord, '#');
            let mut pos = start_pos.clone();
            let mut dir = 0;
            let mut seen: HashSet<(Coord, usize)> = HashSet::new();
            loop {
                let next_step = dirs[dir].step(pos);
                if seen.contains(&(pos, dir)) {
                    total += 1;
                    break;
                }
                seen.insert((pos, dir));
                match &grid.get(&next_step) {
                    Some('#') => dir = (dir + 1) % 4,
                    Some(_c) => pos = next_step,
                    None => break,
                }
            }
            grid.set(&coord, 'X');
        }

        total.to_string()
    }
}

fn get_grid(data: String) -> (Vec2D<char>, Coord) {
    let cols = data.lines().next().unwrap().len();
    let rows = data.lines().count();

    let mut grid = Vec2D::<char>::new(rows, cols);
    let mut start_pos: Coord = Coord::new(0, 0);

    for (row, line) in data.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            grid.set(&Coord::from_usize(row, col), ch);

            if ch == '^' {
                start_pos = Coord::from_usize(row, col);
            }
        }
    }

    (grid, start_pos)
}
