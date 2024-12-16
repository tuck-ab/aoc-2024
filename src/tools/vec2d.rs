use std::default;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coord {
    pub row: i32,
    pub col: i32,
}

impl Coord {
    pub fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }

    pub fn from_usize(row: usize, col: usize) -> Self {
        Self {
            row: row as i32,
            col: col as i32,
        }
    }

    pub fn apply(mut self, dr: i32, dc: i32) -> Self {
        self.row += dr;
        self.col += dc;
        self
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub enum Dir4 {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl Dir4 {
    pub fn step(&self, coord: Coord) -> Coord {
        match self {
            Self::Up => coord.apply(-1, 0),
            Self::Right => coord.apply(0, 1),
            Self::Down => coord.apply(1, 0),
            Self::Left => coord.apply(0, -1),
        }
    }

    pub fn step_back(&self, coord: Coord) -> Coord {
        match self {
            Self::Up => coord.apply(1, 0),
            Self::Right => coord.apply(0, -1),
            Self::Down => coord.apply(-1, 0),
            Self::Left => coord.apply(0, 1),
        }
    }

    pub fn is_vert(&self) -> bool {
        match *self {
            Self::Up | Self::Down => true,
            Self::Left | Self::Right => false,
        }
    }

    pub fn rotate(&self) -> Self {
        match *self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    pub fn rotations(&self, other: &Self) -> i32 {
        (*other as i32 - *self as i32).rem_euclid(4)
    }
}

pub const DIR4S: [Dir4; 4] = [Dir4::Up, Dir4::Right, Dir4::Down, Dir4::Left];

// pub enum Dir8 {
//     Up,
//     UpRight,
//     Right,
//     DownRight,
//     Down,
//     DownLeft,
//     Left,
//     UpLeft,
// }

// pub const DIR8S: [Dir8; 8] = [
//     Dir8::Up,
//     Dir8::UpRight,
//     Dir8::Right,
//     Dir8::DownRight,
//     Dir8::Down,
//     Dir8::DownLeft,
//     Dir8::Left,
//     Dir8::UpLeft,
// ];

#[derive(Clone)]
pub struct Vec2D<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T> Vec2D<T>
where
    T: default::Default,
{
    pub fn new(rows: usize, cols: usize) -> Self {
        let cap = rows * cols;
        let mut data: Vec<T> = Vec::with_capacity(cap);
        for _ in 0..(cap) {
            data.push(T::default())
        }
        Self { data, rows, cols }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn data(&self) -> &Vec<T> {
        &self.data
    }

    pub fn set(&mut self, coord: &Coord, val: T) {
        let i = coord.row as usize * self.cols;
        self.data[i + coord.col as usize] = val
    }

    pub fn in_grid(&self, coord: &Coord) -> bool {
        (coord.row as usize) < self.rows && (coord.col as usize) < self.cols
    }

    pub fn get(&self, coord: &Coord) -> Option<&T> {
        (usize::try_from(coord.row).is_ok()
            && usize::try_from(coord.col).is_ok()
            && self.in_grid(coord))
        .then(|| self.get_index(coord))
    }

    pub fn get_index(&self, coord: &Coord) -> &T {
        let i = coord.row as usize * self.cols;
        &self.data[i + coord.col as usize]
    }

    pub fn get_coord(&self, index: usize) -> Coord {
        Coord {
            row: (index / self.cols) as i32,
            col: (index % self.cols) as i32,
        }
    }
}
