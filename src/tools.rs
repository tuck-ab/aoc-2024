use std::fs;

pub fn load_input(day: u8) -> String {
    fs::read_to_string(format!("inputs/day{}.txt", day)).expect("Could not open input file")
}
