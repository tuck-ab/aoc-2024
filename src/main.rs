use std::time::Instant;

use clap::Parser;

mod tools;

mod day_0;
mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

/// Args doc comment
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Puzzle day
    #[arg(short, long)]
    day: u8,

    /// Puzzle part
    #[arg(short, long)]
    part: u8,
}

pub(crate) trait Solution {
    fn part_1() -> String;
    fn part_2() -> String;
}

fn main() {
    let args = Args::parse();

    let start_time = Instant::now();
    let answer = match args.day {
        0 => run::<day_0::Day0>(args.part),
        1 => run::<day_1::Day1>(args.part),
        2 => run::<day_2::Day2>(args.part),
        3 => run::<day_3::Day3>(args.part),
        4 => run::<day_4::Day4>(args.part),
        5 => run::<day_5::Day5>(args.part),
        6 => run::<day_6::Day6>(args.part),
        7 => run::<day_7::Day7>(args.part),
        8 => run::<day_8::Day8>(args.part),
        9 => run::<day_9::Day9>(args.part),
        _ => panic!("Day '{}' not implemented", args.day),
    };
    let time_taken = start_time.elapsed().as_secs_f32();

    println!("\n=== Day {}: Part {} ===", args.day, args.part);
    println!("Solution: {}", answer);
    println!("Duration: {:.5} seconds", time_taken);
}

fn run<T: Solution>(part: u8) -> String {
    match part {
        1 => T::part_1(),
        2 => T::part_2(),
        _ => panic!("Unexpected part '{}'", part),
    }
}
