use std::cmp::max;

use itertools::Itertools;
use regex::Regex;

use crate::tools::load_input;
use crate::Solution;

pub(crate) struct Day14;

impl Solution for Day14 {
    fn part_1() -> String {
        let data = load_input(14);
        let width = 101;
        let height = 103;
        let seconds = 100;

        let middle_col = width / 2;
        let middle_row = height / 2;

        let mut quadrants = [0, 0, 0, 0];

        for line in data.lines() {
            let mut robot = Robot::new(line);

            robot.x = (robot.x + robot.dx * seconds).rem_euclid(width);
            robot.y = (robot.y + robot.dy * seconds).rem_euclid(height);

            if robot.x != middle_col && robot.y != middle_row {
                let index = 1 * (robot.x < middle_col) as i32 + 2 * (robot.y < middle_row) as i32;
                quadrants[index as usize] += 1
            }
        }
        quadrants.iter().product::<i32>().to_string()
    }

    fn part_2() -> String {
        let data = load_input(14);

        let mut robots: Vec<Robot> = data.lines().map(|s| Robot::new(s)).collect();
        let num_robots = robots.len();
        let mut counter = 0;
        let threashold = 16;

        loop {
            for robot in robots.iter_mut() {
                robot.step()
            }
            counter += 1;

            let c_x = robots.iter().map(|r| r.x).sum::<i32>() / num_robots as i32;
            let c_y = robots.iter().map(|r| r.y).sum::<i32>() / num_robots as i32;

            let score = robots
                .iter()
                .filter(|r| r.distance(c_x, c_y) < threashold)
                .count();

            if score as f32 / num_robots as f32 > 0.6 {
                // print_image(&robots);
                break;
            }
        }
        counter.to_string()
    }
}

struct Robot {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

impl Robot {
    fn new(data: &str) -> Self {
        let re = Regex::new(r"p=([0-9]+),([0-9]+) v=(-?[0-9]+),(-?[0-9]+)").unwrap();
        let [x, y, dx, dy] = re
            .captures_iter(data)
            .map(|c| c.extract())
            .map(|(_, v)| v)
            .exactly_one()
            .expect("More than one match found")
            .map(|n| n.parse::<i32>().expect("Could not parse number"));
        Self { x, y, dx, dy }
    }

    fn step(&mut self) {
        self.x = (self.x + self.dx).rem_euclid(WIDTH as i32);
        self.y = (self.y + self.dy).rem_euclid(HEIGHT as i32);
    }

    /// L0 distance
    fn distance(&self, c_x: i32, c_y: i32) -> i32 {
        max((self.x - c_x).abs(), (self.y - c_y).abs())
    }
}

const WIDTH: usize = 101;
const HEIGHT: usize = 103;

#[allow(dead_code)]
fn print_image(robots: &Vec<Robot>) {
    let mut image: [[char; WIDTH]; HEIGHT] = [[' '; WIDTH]; HEIGHT];

    for robot in robots {
        image[robot.y as usize][robot.x as usize] = 'X';
    }

    for line in image {
        for c in line {
            print!("{}", c);
        }
        println!("");
    }
}
