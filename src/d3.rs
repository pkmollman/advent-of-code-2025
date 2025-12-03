use std::collections::VecDeque;

pub struct PuzzleResult {
    pub joltages: Vec<u64>,
}

impl PuzzleResult {
    pub fn process_input(input: String) -> PuzzleResult {
        let mut result = PuzzleResult {
            joltages: Vec::new(),
        };

        for line in input.lines() {
            let mut front: u64 = 0;
            let mut back: u64 = 0;
            let joltage_values: Vec<u64> = line
                .chars()
                .map(|x| x.to_string().parse().expect("can't parse into int!"))
                .collect();
            for i in 0..joltage_values.len() {
                if joltage_values[i] > front && i < joltage_values.len() - 1 {
                    front = joltage_values[i];
                    continue;
                }
                if joltage_values[i] > back && front != 0 {
                    back = joltage_values[i];
                }
            }

            println!("{} {}", front, back);

            result.joltages.push((front << 10) + back);
        }

        return result;
    }
}
