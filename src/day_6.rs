use std::collections::HashSet;

use crate::tools::{load_input, Vec2D};
use crate::Solution;

pub(crate) struct Day6;

impl Solution for Day6 {
    fn part_1() {
        let data = load_input(6);
        let (mut grid, mut pos) = get_grid(data);
        let mut dir = 0;

        let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        grid.set(pos.0 as usize, pos.1 as usize, 'X');

        loop {
            let next_step = (pos.0 + dirs[dir].0, pos.1 + dirs[dir].1);

            match &grid.get(next_step.0, next_step.1) {
                Some('#') => dir = (dir + 1) % 4,
                Some(_c) => {
                    grid.set(next_step.0 as usize, next_step.1 as usize, 'X');
                    pos = next_step;
                }
                None => break,
            }
        }

        let total = grid.data().iter().filter(|c| **c == 'X').count();
        println!("{}", total)
    }

    fn part_2() {
        let data = load_input(6);
        let (mut grid, start_pos) = get_grid(data);

        let mut dir = 0;
        let mut pos = start_pos.clone();

        let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];

        loop {
            let next_step = (pos.0 + dirs[dir].0, pos.1 + dirs[dir].1);
            match &grid.get(next_step.0, next_step.1) {
                Some('#') => dir = (dir + 1) % 4,
                Some(_c) => {
                    grid.set(next_step.0 as usize, next_step.1 as usize, 'X');
                    pos = next_step;
                }
                None => break,
            }
        }

        let path_points: Vec<(usize, usize)> = grid
            .data()
            .iter()
            .enumerate()
            .filter(|(_, c)| **c == 'X')
            .map(|(i, _)| grid.get_loc(i))
            .collect();

        let mut total = 0;
        for (row, col) in path_points {
            grid.set(row, col, '#');
            let mut pos = start_pos.clone();
            let mut dir = 0;
            let mut seen: HashSet<((i32, i32), usize)> = HashSet::new();
            loop {
                let next_step = (pos.0 + dirs[dir].0, pos.1 + dirs[dir].1);
                if seen.contains(&(pos, dir)) {
                    total += 1;
                    break;
                }
                seen.insert((pos, dir));
                match &grid.get(next_step.0, next_step.1) {
                    Some('#') => dir = (dir + 1) % 4,
                    Some(_c) => pos = next_step,
                    None => break,
                }
            }
            grid.set(row, col, 'X');
        }

        println!("{}", total);
    }
}

fn get_grid(data: String) -> (Vec2D<char>, (i32, i32)) {
    let cols = data.lines().next().unwrap().len();
    let rows = data.lines().count();

    let mut grid = Vec2D::<char>::new(rows, cols);
    let mut start_pos: (i32, i32) = (0, 0);

    for (row, line) in data.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            grid.set(row, col, ch);

            if ch == '^' {
                start_pos = (row as i32, col as i32)
            }
        }
    }

    (grid, start_pos)
}
