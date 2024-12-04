use std::fs;
use std::fmt::Debug;
use std::clone::Clone;
use std::default::Default;

pub fn load_input(day: u8) -> String {
    fs::read_to_string(format!("inputs/day{}.txt", day)).expect("Could not open input file")
}


#[derive(Debug)]
pub struct Vec2D<T> {
    pub data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T> Vec2D<T> where T: Default + Clone + Debug {
    pub fn new(rows: usize, cols: usize) -> Self {
        // let mut data = Vec::<T>::with_capacity(rows * cols);
        // data.fill_with(Default::default);

        // println!("{}", data.len());
        Self {
            data: vec![T::default(); rows*cols],
            rows,
            cols,
        }
    }

    pub fn set(&mut self, row:usize, col: usize, val: T) {
        let i = row * self.cols;
        self.data[i + col] = val
    }

    pub fn get(&self, row: i32, col: i32) -> Option<&T> {
        match (usize::try_from(row), usize::try_from(col)) {
            (Ok(row_usize), Ok(col_usize)) => self
                .in_grid(row_usize, col_usize)
                .then(|| self.index(row_usize, col_usize)),
            _ => None,
        }
    }

    pub fn in_grid(&self, row: usize, col: usize) -> bool {
        row < self.rows && col < self.cols
    }

    pub fn index(&self, row: usize, col: usize) -> &T {
        let i = row * self.cols;
        &self.data[i + col]
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }
}
