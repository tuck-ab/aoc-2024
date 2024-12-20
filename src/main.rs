use std::env::set_var;
use std::time::Instant;

use clap::Parser;

mod tools;

mod day_0;
mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_20;

/// AOC 2024 solutions
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Puzzle day
    #[arg(short, long)]
    day: u8,

    /// Puzzle part
    #[arg(short, long)]
    part: u8,

    /// Use demo input
    #[arg(long)]
    debug: bool,
}

pub(crate) trait Solution {
    fn part_1() -> String;
    fn part_2() -> String;
}

fn main() {
    let args = Args::parse();

    if args.debug {
        set_var("DEBUG", "true");
    }

    let start_time = Instant::now();
    let answer = run(args.day, args.part);
    let time_taken = start_time.elapsed().as_secs_f32();

    println!("\n=== Day {}: Part {} ===", args.day, args.part);
    println!("Solution: {}", answer);
    println!("Duration: {:.5} seconds", time_taken);
}

pub fn run(day: u8, part: u8) -> String {
    match day {
        0 => run_part::<day_0::Day0>(part),
        1 => run_part::<day_01::Day1>(part),
        2 => run_part::<day_02::Day2>(part),
        3 => run_part::<day_03::Day3>(part),
        4 => run_part::<day_04::Day4>(part),
        5 => run_part::<day_05::Day5>(part),
        6 => run_part::<day_06::Day6>(part),
        7 => run_part::<day_07::Day7>(part),
        8 => run_part::<day_08::Day8>(part),
        9 => run_part::<day_09::Day9>(part),
        10 => run_part::<day_10::Day10>(part),
        11 => run_part::<day_11::Day11>(part),
        12 => run_part::<day_12::Day12>(part),
        13 => run_part::<day_13::Day13>(part),
        14 => run_part::<day_14::Day14>(part),
        15 => run_part::<day_15::Day15>(part),
        16 => run_part::<day_16::Day16>(part),
        17 => run_part::<day_17::Day17>(part),
        18 => run_part::<day_18::Day18>(part),
        19 => run_part::<day_19::Day19>(part),
        20 => run_part::<day_20::Day20>(part),
        _ => panic!("Day '{}' not implemented", day),
    }
}

fn run_part<T: Solution>(part: u8) -> String {
    match part {
        1 => T::part_1(),
        2 => T::part_2(),
        _ => panic!("Unexpected part '{}'", part),
    }
}

#[cfg(test)]
mod tests;
