pub struct Result {
    pub invalid_ids: Vec<i64>,
    pub invalid_ids_2: Vec<i64>,
}

fn is_invalid_id(s: &String, subs: String) -> bool {
    // if the substring can't fit into the parent evenly abort
    if s.chars().count() % subs.chars().count() != 0 {
        return false;
    }

    for i in 0..s.chars().count() / subs.chars().count() {
        let scan_buffer: String = s
            .chars()
            .skip(i * subs.chars().count())
            .take(subs.chars().count())
            .collect();
        if scan_buffer != subs {
            return false;
        }
    }

    return true;
}

impl Result {
    pub fn process_input(input: String) -> Result {
        let mut result_data = Result {
            invalid_ids: Vec::new(),
            invalid_ids_2: Vec::new(),
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

                // handle part 1
                if id_string.chars().count() % 2 == 0 {
                    let first_half: String = id_string
                        .chars()
                        .take(id_string.chars().count() / 2)
                        .collect();
                    let second_half: String = id_string
                        .chars()
                        .skip(id_string.chars().count() / 2)
                        .take(id_string.chars().count() / 2)
                        .collect();
                    if first_half == second_half {
                        result_data.invalid_ids.push(id);
                    }
                }

                // handle part 2
                let mut found_invalid_combo = false;
                for cc in 1..=id_string.chars().count() / 2 {
                    found_invalid_combo =
                        is_invalid_id(&id_string, id_string.chars().take(cc).collect::<String>());
                    if found_invalid_combo {
                        result_data.invalid_ids_2.push(id);
                        break;
                    }
                }
            }
        }
        result_data
    }
}
