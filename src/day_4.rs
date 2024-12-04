use crate::tools::{load_input, Vec2D};
use crate::Solution;

pub(crate) struct Day4;

impl Solution for Day4 {
    fn part_1() {
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
                if *grid.index(row, col) == 'X' {
                    for (dr, dc) in dirs {
                        for (i, check_c) in check.chars().enumerate() {
                            let c = match grid
                                .get(row as i32 + (dr * i as i32), col as i32 + (dc * i as i32))
                            {
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

        println!("{}", total)
    }

    fn part_2() {
        let data = load_input(4);
        let grid = get_grid(data);

        let mut total = 0;

        for row in 0..grid.rows() {
            for col in 0..grid.cols() {
                if *grid.index(row, col) == 'A' {
                    let diag1 = match (
                        grid.get(row as i32 + 1, col as i32 - 1),
                        grid.get(row as i32 - 1, col as i32 + 1),
                    ) {
                        (Some(c1), Some(c2)) => {
                            (*c1 == 'M' && *c2 == 'S') || (*c1 == 'S' && *c2 == 'M')
                        }
                        _ => false,
                    };

                    let diag2 = match (
                        grid.get(row as i32 - 1, col as i32 - 1),
                        grid.get(row as i32 + 1, col as i32 + 1),
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

        println!("{}", total)
    }
}

fn get_grid(data: String) -> Vec2D<char> {
    let cols = data.lines().next().unwrap().len();
    let rows = data.lines().count();

    let mut grid = Vec2D::<char>::new(rows, cols);

    for (row, line) in data.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            grid.set(row, col, ch);
        }
    }

    grid
}
