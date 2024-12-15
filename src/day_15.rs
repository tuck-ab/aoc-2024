use std::collections::BTreeSet;

use crate::tools::{load_input, Coord, Dir4, Vec2D};
use crate::Solution;

pub(crate) struct Day15;

impl Solution for Day15 {
    fn part_1() -> String {
        let data = load_input(15);
        let (mut grid, dirs, mut robot_pos) = parse_data(data);

        for dir in dirs {
            let target_pos = dir.step(robot_pos.clone());

            match grid
                .get(&target_pos)
                .expect(&format!("Couldn't find grid coord '{:?}'", target_pos))
            {
                Item::Wall => {}
                Item::Empty => {
                    grid.set(&robot_pos, Item::Empty);
                    grid.set(&target_pos, Item::Robot);
                    robot_pos = target_pos;
                }
                Item::Robot => {
                    panic! {"Robot trying to move into itself"}
                }
                Item::Box(_) => {
                    let mut scan_pos = target_pos.clone();
                    scan_pos = dir.step(scan_pos);
                    while *grid.get(&scan_pos).unwrap() == Item::Box(true) {
                        scan_pos = dir.step(scan_pos)
                    }
                    if *grid.get(&scan_pos).unwrap() == Item::Empty {
                        grid.set(&scan_pos, Item::Box(true));
                        grid.set(&robot_pos, Item::Empty);
                        grid.set(&target_pos, Item::Robot);
                        robot_pos = target_pos;
                    }
                }
            }
        }

        grid.data()
            .iter()
            .enumerate()
            .filter(|(_, c)| **c == Item::Box(true))
            .map(|(i, _)| {
                let c = grid.get_coord(i);
                100 * c.row + c.col
            })
            .sum::<i32>()
            .to_string()
    }

    fn part_2() -> String {
        let data = load_input(15);
        let (small_grid, dirs, mut robot_pos) = parse_data(data);

        // Transform the grid
        let mut grid: Vec2D<Item> = Vec2D::new(small_grid.rows(), small_grid.cols() * 2);
        for (i, c) in small_grid.data().iter().enumerate() {
            let mut coord = small_grid.get_coord(i);
            coord.col *= 2;
            match *c {
                Item::Empty => {
                    grid.set(&coord, Item::Empty);
                    coord.col += 1;
                    grid.set(&coord, Item::Empty);
                }
                Item::Wall => {
                    grid.set(&coord, Item::Wall);
                    coord.col += 1;
                    grid.set(&coord, Item::Wall);
                }
                Item::Box(_) => {
                    grid.set(&coord, Item::Box(true));
                    coord.col += 1;
                    grid.set(&coord, Item::Box(false));
                }
                Item::Robot => {
                    grid.set(&coord, Item::Robot);
                    coord.col += 1;
                    grid.set(&coord, Item::Empty);
                }
            }
        }
        robot_pos.col *= 2;

        for dir in dirs.iter() {
            let target_pos = dir.step(robot_pos.clone());

            match grid
                .get(&target_pos)
                .expect(&format!("Couldn't find grid coord '{:?}'", target_pos))
            {
                Item::Wall => {}
                Item::Empty => {
                    grid.set(&robot_pos, Item::Empty);
                    grid.set(&target_pos, Item::Robot);
                    robot_pos = target_pos;
                }
                Item::Robot => {
                    panic! {"Robot trying to move into itself"}
                }
                Item::Box(_) => {
                    let mut scan_pos_vec = vec![target_pos.clone()];
                    let mut can_move: bool = true;
                    let mut boxes: BTreeSet<Coord> = BTreeSet::new();

                    'outer: while !scan_pos_vec.is_empty() {
                        let mut next_scan_pos_set: BTreeSet<Coord> = BTreeSet::new();
                        for scan_pos in scan_pos_vec.iter() {
                            let mut next_tile = dir.step(scan_pos.clone());
                            match *grid.get(&scan_pos).unwrap() {
                                Item::Wall => {
                                    can_move = false;
                                    break 'outer;
                                }
                                Item::Empty => {}
                                Item::Robot => {
                                    panic!("Robot trying to move into iteslf");
                                }
                                Item::Box(lr) => {
                                    if !dir.is_vert() {
                                        boxes.insert(scan_pos.clone());
                                        boxes.insert(next_tile.clone());
                                        next_tile = dir.step(next_tile);
                                        next_scan_pos_set.insert(next_tile);
                                    } else {
                                        boxes.insert(scan_pos.clone());
                                        let mut other_half = scan_pos.clone();
                                        if lr {
                                            other_half.col += 1
                                        } else {
                                            other_half.col -= 1
                                        }
                                        boxes.insert(other_half.clone());
                                        next_scan_pos_set.insert(dir.step(other_half));
                                        next_scan_pos_set.insert(next_tile.clone());
                                        if *grid.get(&next_tile).unwrap() == Item::Box(!lr) {
                                            // If the box is the left side then add to the left
                                            // []. // Next tile
                                            // .[] // Scan pos
                                            // .@. // Robot pos
                                            if lr {
                                                next_tile.col -= 1
                                            } else {
                                                next_tile.col += 1
                                            }
                                            next_scan_pos_set.insert(next_tile.clone());
                                        }
                                    }
                                }
                            }
                        }
                        scan_pos_vec = next_scan_pos_set.into_iter().collect();
                    }

                    if can_move {
                        let mut boxes_vec: Vec<Coord> = boxes.into_iter().collect();
                        match *dir {
                            Dir4::Up => boxes_vec.sort_by(|c1, c2| c1.row.cmp(&c2.row)),
                            Dir4::Down => boxes_vec.sort_by(|c1, c2| c2.row.cmp(&c1.row)),
                            Dir4::Left => boxes_vec.sort_by(|c1, c2| c1.col.cmp(&c2.col)),
                            Dir4::Right => boxes_vec.sort_by(|c1, c2| c2.col.cmp(&c1.col)),
                        }
                        for box_coord in boxes_vec.iter() {
                            match *grid.get(&box_coord).unwrap() {
                                Item::Box(lr) => {
                                    grid.set(&box_coord, Item::Empty);
                                    grid.set(&dir.step(box_coord.clone()), Item::Box(lr))
                                }
                                _ => panic!(
                                    "Coord gives unexpected value '{:?}'",
                                    *grid.get(&box_coord).unwrap()
                                ),
                            }
                        }
                        grid.set(&robot_pos, Item::Empty);
                        grid.set(&target_pos, Item::Robot);
                        robot_pos = target_pos;
                    }
                }
            }
        }

        grid.data()
            .iter()
            .enumerate()
            .filter(|(_, c)| **c == Item::Box(true))
            .map(|(i, _)| {
                let c = grid.get_coord(i);
                100 * c.row + c.col
            })
            .sum::<i32>()
            .to_string()
    }
}

#[allow(dead_code)]
fn print_grid(grid: &Vec2D<Item>) {
    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            print!(
                "{}",
                grid.get(&Coord::from_usize(row, col)).unwrap().get_char()
            )
        }
        println!("");
    }
}

#[derive(Default, PartialEq, Eq, Debug, Clone)]
enum Item {
    #[default]
    Empty,
    Wall,
    /// `true` is left side of box
    Box(bool),
    Robot,
}

impl Item {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::Wall,
            'O' => Self::Box(true),
            '@' => Self::Robot,
            _ => panic!("Unknown symbol '{}'", c),
        }
    }

    fn get_char(&self) -> char {
        match *self {
            Self::Empty => '.',
            Self::Robot => '@',
            Self::Wall => '#',
            Self::Box(true) => '[',
            Self::Box(false) => ']',
        }
    }
}

fn dir_from_arrow(c: char) -> Dir4 {
    match c {
        '^' => Dir4::Up,
        '>' => Dir4::Right,
        'v' => Dir4::Down,
        '<' => Dir4::Left,
        _ => panic!("Unknown direction symbol '{}'", c),
    }
}

fn parse_data(data: String) -> (Vec2D<Item>, Vec<Dir4>, Coord) {
    let parts: Vec<String> = data.split("\n\n").map(|s| s.to_string()).collect();

    let cols = parts[0].lines().next().unwrap().len();
    let rows = parts[0].lines().count();
    let mut grid: Vec2D<Item> = Vec2D::new(rows, cols);
    let mut robot_pos: Coord = Coord::new(0, 0);

    for (row, line) in parts[0].lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            let coord = Coord::from_usize(row, col);
            grid.set(&coord, Item::from_char(c));
            if c == '@' {
                robot_pos = coord;
            }
        }
    }

    let dirs: Vec<Dir4> = parts[1]
        .lines()
        .map(|l| l.chars().map(|c| dir_from_arrow(c)).collect())
        .collect::<Vec<Vec<Dir4>>>()
        .concat();

    (grid, dirs, robot_pos)
}
