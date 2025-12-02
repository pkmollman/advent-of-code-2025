pub struct Result {
    pub invalid_ids: Vec<i64>,
}

impl Result {
    pub fn process_input(input: String) -> Result {
        let mut result_data = Result {
            invalid_ids: Vec::new(),
        };
        let line_with_ids = input
            .lines()
            .nth(0)
            .expect("Could not parse a first line for puzzle input");
        for range_string in line_with_ids.split(",") {
            let mut start_string = String::new();
            let mut end_string = String::new();
            let mut hyphen_seen = false;
            for ch in range_string.chars() {
                match ch {
                    '-' => {
                        hyphen_seen = true;
                    }
                    _ => {
                        if hyphen_seen {
                            end_string.push(ch);
                        } else {
                            start_string.push(ch);
                        }
                    }
                }
            }
            let start: i64 = start_string
                .parse()
                .expect("could not parse start string to int");
            let end: i64 = end_string
                .parse()
                .expect("could not parse end string to int");

            for id in start..=end {
                let id_string = id.to_string();
                if id_string.chars().count() % 2 == 0 {
                    let mut first_half: String = id_string
                        .chars()
                        .take(id_string.chars().count() / 2)
                        .collect();
                    let mut second_half: String = id_string
                        .chars()
                        .skip(id_string.chars().count() / 2)
                        .take(id_string.chars().count() / 2)
                        .collect();
                    if first_half == second_half {
                        result_data.invalid_ids.push(id);
                    }
                }
            }
        }
        result_data
    }
}
