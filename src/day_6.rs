use std::collections::{HashMap, HashSet};

use sorted_vec::SortedVec;

use crate::tools::{load_demo, load_input, Vec2D};
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
        let data = load_demo(6);
        let (mut grid, start_pos) = get_grid(data);
        let obs: Obstacles = Obstacles::new(&grid);
    }
}

impl Day6 {
    fn part_2() {
        let data = load_demo(6);
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

struct Obstacles {
    by_col: HashMap<usize, SortedVec<usize>>,
    by_row: HashMap<usize, SortedVec<usize>>,
}

enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

impl Direction {
    fn next(&self) -> Self {
        match self {
            Self::UP => Self::RIGHT,
            Self::RIGHT => Self::DOWN,
            Self::DOWN => Self::LEFT,
            Self::LEFT => Self::UP,
        }
    }

    fn get_changing_val(&self, loc: (usize, usize)) -> usize {
        match *self {
            Self::DOWN | Self::UP => loc.0,
            Self::LEFT | Self::RIGHT => loc.1,
        }
    }

    fn get_static_val(&self, loc: (usize, usize)) -> usize {
        match *self {
            Self::DOWN | Self::UP => loc.1,
            Self::LEFT | Self::RIGHT => loc.0,
        }
    }

    fn step(&self, loc: (usize, usize)) -> (usize, usize) {
        match *self {
            Self::UP => (loc.0 - 1, loc.1),
            Self::RIGHT => (loc.0, loc.1 + 1),
            Self::DOWN => (loc.0 + 1, loc.1),
            Self::LEFT => (loc.1, loc.1 - 1),
        }
    }

    fn step_back(&self, loc: (usize, usize)) -> (usize, usize) {
        match *self {
            Self::UP => (loc.0 + 1, loc.1),
            Self::RIGHT => (loc.0, loc.1 - 1),
            Self::DOWN => (loc.0 - 1, loc.1),
            Self::LEFT => (loc.1, loc.1 + 1),
        }
    }

    fn is_off(&self, index: usize, vec_len: usize) -> bool {
        match *self {
            Self::UP | Self::LEFT => index == 0,
            Self::RIGHT | Self::DOWN => index == vec_len,
        }
    }

    fn index_morph(&self, index: usize) -> usize {
        match *self {
            Self::UP | Self::LEFT => index - 1,
            Self::RIGHT | Self::DOWN => index,
        }
    }

    fn get_loc(&self, static_loc: usize, changing_loc: usize) -> (usize, usize) {
        match *self {
            Self::DOWN | Self::UP => (changing_loc, static_loc),
            Self::LEFT | Self::RIGHT => (static_loc, changing_loc),
        }
    }
}

impl Obstacles {
    fn new(data: &Vec2D<char>) -> Self {
        let mut by_col: HashMap<usize, SortedVec<usize>> = HashMap::new();
        let mut by_row: HashMap<usize, SortedVec<usize>> = HashMap::new();

        for (i, c) in data.data().iter().enumerate().filter(|(_, cc)| **cc == '#') {
            let (row, col) = data.get_loc(i);
            by_row.entry(row).or_default().insert(col);
            by_col.entry(col).or_default().insert(row);
        }

        Self { by_col, by_row }
    }

    fn get_next_target(&self, pos: (usize, usize), dir: Direction) -> Option<(usize, usize)> {
        let by_x = match match dir {
            Direction::DOWN | Direction::UP => &self.by_col,
            Direction::LEFT | Direction::RIGHT => &self.by_row,
        }
        .get(&dir.get_changing_val(pos))
        {
            Some(x) => x,
            None => return None,
        };

        let index = match by_x.binary_search(&dir.get_static_val(pos)) {
            Ok(_) => panic!("Standing on an obtacle"),
            Err(i) => i,
        };

        if dir.is_off(index, by_x.len()) {
            return None;
        } else {
            Some(dir.get_loc(dir.get_static_val(pos), dir.index_morph(index)))
        }
    }
}
