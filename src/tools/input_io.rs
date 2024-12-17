use std::env;
use std::fs;

pub fn load_input(day: u8) -> String {
    let debug = env::var("DEBUG").unwrap_or("false".to_string());
    if debug == "true" {
        fs::read_to_string(format!("demos/day{}.txt", day)).expect("Could not open demo file")
    } else {
        fs::read_to_string(format!("inputs/day{}.txt", day)).expect("Could not open input file")
    }
}
