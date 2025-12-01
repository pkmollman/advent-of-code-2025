pub mod d1;

use std::fs;

fn load_puzzle_input(file_name: &str) -> String {
    fs::read_to_string(format!("./puzzle_inputs/{}", file_name)).unwrap()
}

fn main() {
    println!("==== DAY 1 ====");
    let day_1 = d1::Result1::process_input(load_puzzle_input("1.txt"), 50);
    println!("times at zero: {}", day_1.times_reached_0);
    println!("times passed zero: {}", day_1.times_passed_0);
}
