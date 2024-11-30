mod tools;

mod day_0;

use clap::Parser;

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
    fn part_1();
    fn part_2();
}

fn main() {
    let args = Args::parse();

    match args.day {
        0 => run::<day_0::Day0>(args.part),
        _ => panic!("Day '{}' not implemented", args.day),
    }
}

fn run<T: Solution>(part: u8) {
    match part {
        1 => T::part_1(),
        2 => T::part_2(),
        _ => panic!("Unexpected part '{}'", part),
    }
}