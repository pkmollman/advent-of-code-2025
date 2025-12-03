use std::collections::VecDeque;

pub struct PuzzleResult {
    pub joltages: Vec<u64>,
    pub joltages_2: Vec<u64>,
}

impl PuzzleResult {
    pub fn process_input(input: String) -> PuzzleResult {
        let mut result = PuzzleResult {
            joltages: Vec::new(),
            joltages_2: Vec::new(),
        };

        for line in input.lines() {
            let joltage_values: Vec<u64> = line
                .chars()
                .map(|x| x.to_string().parse().expect("can't parse into int!"))
                .collect();

            // PART 1
            let mut front: u64 = 0;
            let mut front_index: usize = 0;
            let mut back: u64 = 0;

            for i in 0..joltage_values.len() {
                if joltage_values[i] > front && i < joltage_values.len() - 1 {
                    front = joltage_values[i];
                    front_index = i;
                }
            }

            for i in 0..joltage_values.len() {
                if joltage_values[i] > back && i > front_index {
                    back = joltage_values[i];
                }
            }

            result.joltages.push(front * 10 + back);

            // PART 2
            let mut found_values: Vec<u64> = Vec::new();
            let mut scan_index: usize = 0;

            for i in 0..12 {
                let mut remaining_front_slice: Vec<u64> = joltage_values
                    .iter()
                    .skip(scan_index)
                    .take(joltage_values.len() - scan_index - (12 - i) + 1)
                    .copied()
                    .collect();
                remaining_front_slice.sort();
                found_values.push(remaining_front_slice.last().unwrap().to_owned());
                for ia in scan_index..joltage_values.len() {
                    if &joltage_values[ia] == found_values.last().unwrap() {
                        scan_index = ia + 1;
                        break;
                    }
                }
            }

            let mut total: u64 = 0;
            for i in 0..found_values.len() {
                total += found_values[i] * u64::pow(10, (found_values.len() - i - 1) as u32);
            }
            result.joltages_2.push(total);
        }

        return result;
    }
}
