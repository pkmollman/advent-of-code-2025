pub struct PuzzleResult {
    pub column_results: i64,
    pub column_results_2: i64,
    y: Vec<Vec<String>>,
}

impl PuzzleResult {
    pub fn process_input(input: String) -> PuzzleResult {
        let mut result = PuzzleResult {
            column_results: 0,
            column_results_2: 0,
            y: Vec::new(),
        };

        for line in input.lines() {
            result
                .y
                .push(line.split_whitespace().map(|x| x.to_owned()).collect());
        }

        // part 1
        for problem_id in 0..result.y[0].len() {
            let mut answer: i64 = result.y[0][problem_id].parse().unwrap();
            for value_id in 1..result.y.len() - 1 {
                match result.y.last().unwrap()[problem_id].as_str() {
                    "+" => {
                        answer += result.y[value_id][problem_id].parse::<i64>().unwrap();
                    }
                    "*" => {
                        answer *= result.y[value_id][problem_id].parse::<i64>().unwrap();
                    }
                    _ => {}
                }
            }
            result.column_results += answer;
        }

        // part 2
        for problem_id in 0..result.y[0].len() {
            let mut longest: isize = 0;
            let mut cepha_nummies: Vec<String> = Vec::new();
            for row_id in 0..result.y.len() - 1 {
                if result.y[row_id][problem_id].len() > longest as usize {
                    longest = result.y[row_id][problem_id].len() as isize;
                }
            }

            // println!("longest: {}", (0..=longest - 1).rev());
            for pos in (0..longest).rev() {
                println!("pos: {}", pos);
                let mut cepha_nummy: String = String::new();
                for row_id in 0..result.y.len() - 1 {
                    if pos < result.y[row_id][problem_id].len() as isize {
                        let chars: Vec<char> = result.y[row_id][problem_id].chars().collect();
                        cepha_nummy.push(chars[pos as usize]);
                    }
                }
                println!("{}", cepha_nummy);
                cepha_nummies.push(cepha_nummy);
            }

            let mut answer: i64 = cepha_nummies[0].parse().unwrap();
            for value_id in 1..cepha_nummies.len() {
                match result.y.last().unwrap()[problem_id].as_str() {
                    "+" => {
                        answer += cepha_nummies[value_id].parse::<i64>().unwrap();
                    }
                    "*" => {
                        answer *= cepha_nummies[value_id].parse::<i64>().unwrap();
                    }
                    _ => {}
                }
            }
            result.column_results_2 += answer;
        }

        return result;
    }
}
