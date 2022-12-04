use std::fs;
use std::env;

pub mod day1;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let input = fs::read_to_string(file_path)
                        .expect("Should have been able to read the file");

    day1::parse_input(input.as_str());
}
