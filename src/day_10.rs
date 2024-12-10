use std::collections::BTreeSet;

use crate::tools::{Vec2D, load_input, Coord, DIR4S};
use crate::Solution;

pub(crate) struct Day10;

impl Solution for Day10 {
    fn part_1() -> String {
        let data = load_input(10);
        let (grid, zero_locs) = get_grid(&data);

        let mut total = 0;

        for start in zero_locs {
            // let mut nine_locs: HashSet<(usize, usize)> = HashSet::new();
            let mut queue: BTreeSet<(u32, Coord)> = BTreeSet::new();
            let mut visited: BTreeSet<Coord> = BTreeSet::new();

            queue.insert((0, start));

            while !queue.is_empty(){
                let (num, loc) = queue.pop_last().unwrap();
                if visited.contains(&loc) {
                    continue;
                }
                visited.insert(loc);

                if num == 9 {
                    total += 1;
                    continue;
                }

                for dir in DIR4S {
                    let n_coord = dir.step(loc);
                    
                    match grid.get(&n_coord) {
                        Some(val) => if *val == num + 1 {queue.insert((*val, n_coord));},
                        None => {}
                    }
                }
            }
        }

        total.to_string()
    }

    fn part_2() -> String {
        let data = load_input(10);
        let (grid, zero_locs) = get_grid(&data);

        let mut total = 0;

        for start in zero_locs {
            // let mut nine_locs: HashSet<(usize, usize)> = HashSet::new();
            let mut stack: Vec<(u32, Coord)> = Vec::new();
            // let mut visited: BTreeSet<(i32, i32)> = BTreeSet::new();

            stack.push((0, start));

            while !stack.is_empty(){
                let (num, loc) = stack.pop().unwrap();
                if num == 9 {
                    total += 1;
                    continue;
                }

                for dir in DIR4S {
                    let n_coord = dir.step(loc);
                    
                    match grid.get(&n_coord) {
                        Some(val) => if *val == num + 1 {stack.push((*val, n_coord));},
                        None => {}
                    }
                }
            }
        }

        total.to_string()
    }
}

// enum Dirs {
//     Up,
//     Down,
//     Left,
//     Right,
// }

// const DIRS: [Dirs; 4] = [Dirs::Up, Dirs::Down, Dirs::Left, Dirs::Right];

// impl Dirs {
//     fn apply(&self, coord: (i32, i32)) -> (i32, i32) {
//         match self {
//             Dirs::Up => (coord.0 - 1, coord.1),
//             Dirs::Down => (coord.0 + 1, coord.1),
//             Dirs::Left => (coord.0, coord.1 -1),
//             Dirs::Right => (coord.0, coord.1 + 1)
//         }
//     }
// }

fn get_grid(data: &String) -> (Vec2D<u32>, Vec<Coord>) {
    let cols = data.lines().next().unwrap().len();
    let rows = data.lines().count();

    let mut grid: Vec2D<u32> = Vec2D::new(rows, cols);
    let mut locs: Vec<Coord> = Vec::new();

    for (row, line) in data.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            let num = ch.to_digit(10).unwrap();
            let coord = Coord::from_usize(row, col);
            grid.set(&coord, num);
            if num == 0 {
                locs.push(coord)
            }
        }
    }

    (grid, locs)
}
