use regex::Regex;

use crate::tools::load_input;
use crate::Solution;

pub(crate) struct Day13;

impl Solution for Day13 {
    fn part_1() -> String {
        let data = load_input(13);
        let games = get_games(data);

        let mut total = 0;

        for game in games {
            let b = ((game.but_a.dx * game.prize_y) / game.but_a.dy - game.prize_x)
                / ((game.but_a.dx * game.but_b.dy) / game.but_a.dy - game.but_b.dx);
            let a = (game.prize_y - game.but_b.dy * b) / game.but_a.dy;

            if is_int(a) && is_int(b) && a > 0.5_f64 && b > 0.5_f64 {
                total += 3 * a.round() as i32 + b.round() as i32;
            }
        }

        total.to_string()
    }

    fn part_2() -> String {
        let data = load_input(13);
        let mut games = get_games(data);

        let mut total = 0;

        for game in games.iter_mut() {
            game.prize_x += 10000000000000.0;
            game.prize_y += 10000000000000.0;
            let b = ((game.but_a.dx * game.prize_y) / game.but_a.dy - game.prize_x)
                / ((game.but_a.dx * game.but_b.dy) / game.but_a.dy - game.but_b.dx);
            let a = (game.prize_y - game.but_b.dy * b) / game.but_a.dy;

            if is_int(a) && is_int(b) && a > 0.5_f64 && b > 0.5_f64 {
                total += 3 * a.round() as i64 + b.round() as i64;
            }
        }

        total.to_string()
    }
}

fn get_games(data: String) -> Vec<Game> {
    data.split("\n\n").map(|s| Game::new(s)).collect()
}

const THREASHOLD: f64 = 0.001;

fn is_int(num: f64) -> bool {
    num.fract() < THREASHOLD || num.fract() > 1_f64 - THREASHOLD
}

#[derive(Debug)]
struct Button {
    dx: f64,
    dy: f64,
}

impl Button {
    fn from_str_data(x: &str, y: &str) -> Self {
        Self {
            dx: x.parse().expect("Could not parse number"),
            dy: y.parse().expect("Could not parse number"),
        }
    }
}

#[derive(Debug)]
struct Game {
    but_a: Button,
    but_b: Button,
    prize_x: f64,
    prize_y: f64,
}

impl Game {
    fn new(data: &str) -> Self {
        let a_reg = Regex::new(r"Button A: X\+([0-9]+), Y\+([0-9]+)").unwrap();
        let b_reg = Regex::new(r"Button B: X\+([0-9]+), Y\+([0-9]+)").unwrap();
        let p_reg = Regex::new(r"Prize: X=([0-9]+), Y=([0-9]+)").unwrap();

        let but_a = a_reg
            .captures_iter(data)
            .map(|c| c.extract())
            .map(|(_, [n1, n2])| Button::from_str_data(n1, n2))
            .next()
            .unwrap();
        let but_b = b_reg
            .captures_iter(data)
            .map(|c| c.extract())
            .map(|(_, [n1, n2])| Button::from_str_data(n1, n2))
            .next()
            .unwrap();
        let (prize_x, prize_y) = p_reg
            .captures_iter(data)
            .map(|c| c.extract())
            .map(|(_, [n1, n2])| {
                (
                    n1.parse::<f64>().expect("Could not parse number"),
                    n2.parse::<f64>().expect("Could  not parse number"),
                )
            })
            .next()
            .unwrap();

        Self {
            but_a,
            but_b,
            prize_x,
            prize_y,
        }
    }
}
