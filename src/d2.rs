pub struct Result2 {
    invalid_ids: i32,
}

impl Result2 {
    pub fn process_input(input: String) -> Result2 {
        let mut result_data = Result2 { invalid_ids: 0 };
        let line_with_ids = input
            .lines()
            .nth(0)
            .expect("Could not parse a first line for input");
        for range_string in line_with_ids.split(",") {
            let mut start_string = String::new();
            let mut end_string = String::new();
            let mut hyphen_seen = false;
            for character in range_string.chars() {
                // m
            }
        }
        result_data
    }
}
