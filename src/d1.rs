pub struct PuzzleResult {
    pub current_position: i32,
    pub times_reached_0: i32,
    pub times_passed_0: i32,
}

impl PuzzleResult {
    fn move_dial(&mut self, dir: char, amount: i32) {
        match dir {
            'L' => {
                for _ in 0..amount {
                    self.current_position -= 1;
                    self.current_position = self.current_position % 100;
                    if self.current_position == 0 {
                        self.times_passed_0 += 1;
                    }
                }
            }
            'R' => {
                for _ in 0..amount {
                    self.current_position += 1;
                    self.current_position = self.current_position % 100;
                    if self.current_position == 0 {
                        self.times_passed_0 += 1;
                    }
                }
            }
            _ => {
                panic!("Bad dial direction: {}", dir)
            }
        }

        if self.current_position == 0 {
            self.times_reached_0 += 1;
        }
    }

    pub fn process_input(input: String, starting_value: i32) -> PuzzleResult {
        let mut result_data = PuzzleResult {
            current_position: starting_value,
            times_reached_0: 0,
            times_passed_0: 0,
        };
        for line in input.lines() {
            let mut dir: Option<char> = None;
            let mut amount_string = String::new();
            for character in line.chars() {
                match dir {
                    None => {
                        dir = Some(character.clone());
                    }
                    Some(_direction) => {
                        amount_string.push(character);
                    }
                }
            }
            result_data.move_dial(dir.unwrap(), amount_string.parse().unwrap());
        }
        result_data
    }
}
