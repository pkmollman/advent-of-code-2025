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
            result.joltages.push(process_joltage(2, &joltage_values));

            // PART 2
            result.joltages_2.push(process_joltage(12, &joltage_values));
        }

        return result;
    }
}

fn process_joltage(target_batteries: usize, joltage_values: &Vec<u64>) -> u64 {
    let mut found_values: Vec<u64> = Vec::new();
    let mut scan_index: usize = 0;

    for i in 0..target_batteries {
        let mut remaining_front_slice: Vec<u64> = joltage_values
            .iter()
            .skip(scan_index)
            .take(joltage_values.len() - scan_index - (target_batteries - i) + 1)
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

    return total;
}
