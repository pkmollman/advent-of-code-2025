mod d1;
mod d2;
mod d3;
mod d4;

use std::fs;

fn load_puzzle_input(file_name: &str) -> String {
    println!("==== DAY {} ====", file_name);
    fs::read_to_string(format!("./puzzle_inputs/{}.txt", file_name)).unwrap()
}

fn main() {
    let day_1 = d1::PuzzleResult::process_input(load_puzzle_input("1"), 50);
    println!("times at zero: {}", day_1.times_reached_0);
    println!("times passed zero: {}", day_1.times_passed_0);

    let day_2 = d2::PuzzleResult::process_input(load_puzzle_input("2"));
    println!("invalid IDs: {}", day_2.invalid_ids.iter().sum::<i64>());
    println!("invalid IDs 2: {}", day_2.invalid_ids_2.iter().sum::<i64>());

    let day_3 = d3::PuzzleResult::process_input(load_puzzle_input("3"));
    println!(
        "total joltage 2 batteries: {}",
        day_3.joltages.iter().sum::<u64>()
    );
    println!(
        "total joltage 12 batteries: {}",
        day_3.joltages_2.iter().sum::<u64>()
    );

    let day_4 = d4::PuzzleResult::process_input(load_puzzle_input("4"));
    println!("accessible rolls: {}", day_4.accessible_rolls);
}
