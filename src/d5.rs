pub struct PuzzleResult {
    pub fresh_values: Vec<u64>,
    pub ingredients: Vec<u64>,
}

impl PuzzleResult {
    pub fn process_input(input: String) -> PuzzleResult {
        let mut result = PuzzleResult {
            fresh_values: Vec::new(),
            ingredients: Vec::new(),
        };

        for line in input.lines() {
            if line.contains('-') {
                let split: Vec<u64> = line.split("-").map(|x| x.parse::<u64>().unwrap()).collect();
                for val in split[0]..=split[1] {
                    result.fresh_values.push(val);
                }
            }
        }

        return result;
    }
}
