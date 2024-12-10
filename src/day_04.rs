use crate::tools::{load_input, Coord, Vec2D};
use crate::Solution;

pub(crate) struct Day4;

impl Solution for Day4 {
    fn part_1() -> String {
        let data = load_input(4);
        let grid = get_grid(data);

        let dirs = [
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
        ];
        let check = "XMAS";

        let mut total = 0;

        for row in 0..grid.rows() {
            for col in 0..grid.cols() {
                let coord = Coord::from_usize(row, col);
                if *grid.get_index(&coord) == 'X' {
                    for (dr, dc) in dirs {
                        for (i, check_c) in check.chars().enumerate() {
                            let c = match grid.get(&Coord::new(
                                coord.row + (dr * i as i32),
                                coord.col + (dc * i as i32),
                            )) {
                                Some(c) => c,
                                None => break,
                            };
                            if *c != check_c {
                                break;
                            }
                            if i == 3 {
                                total += 1;
                            }
                        }
                    }
                }
            }
        }

        total.to_string()
    }

    fn part_2() -> String {
        let data = load_input(4);
        let grid = get_grid(data);

        let mut total = 0;

        for row in 0..grid.rows() {
            for col in 0..grid.cols() {
                let coord = Coord::from_usize(row, col);
                if *grid.get_index(&coord) == 'A' {
                    let diag1 = match (
                        grid.get(&coord.clone().apply(1, -1)),
                        grid.get(&coord.clone().apply(-1, 1)),
                    ) {
                        (Some(c1), Some(c2)) => {
                            (*c1 == 'M' && *c2 == 'S') || (*c1 == 'S' && *c2 == 'M')
                        }
                        _ => false,
                    };

                    let diag2 = match (
                        grid.get(&coord.clone().apply(1, 1)),
                        grid.get(&coord.clone().apply(-1, -1)),
                    ) {
                        (Some(c1), Some(c2)) => {
                            (*c1 == 'M' && *c2 == 'S') || (*c1 == 'S' && *c2 == 'M')
                        }
                        _ => false,
                    };

                    if diag1 && diag2 {
                        total += 1;
                    }
                }
            }
        }

        total.to_string()
    }
}

fn get_grid(data: String) -> Vec2D<char> {
    let cols = data.lines().next().unwrap().len();
    let rows = data.lines().count();

    let mut grid = Vec2D::<char>::new(rows, cols);

    for (row, line) in data.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            grid.set(&Coord::from_usize(row, col), ch);
        }
    }

    grid
}
