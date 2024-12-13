use std::fs;

pub fn load_input(day: u8) -> String {
    fs::read_to_string(format!("inputs/day{}.txt", day)).expect("Could not open input file")
}

#[allow(dead_code)]
pub fn load_demo(day: u8) -> String {
    fs::read_to_string(format!("demos/day{}.txt", day)).expect("Could not open input file")
}
