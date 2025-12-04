pub struct PuzzleResult {
    pub accessible_rolls: u64,
    pub recursive_accessible_rolls: u64,
}

struct Grid {
    rows: Vec<Vec<char>>,
}

impl Grid {
    pub fn add_row(&mut self, row: Vec<char>) {
        self.rows.push(row);
    }

    pub fn get_cell(&self, x: isize, y: isize) -> Option<char> {
        Some(self.rows.get(y as usize)?.get(x as usize)?.to_owned())
    }

    pub fn set_cell(&mut self, x: isize, y: isize, c: char) {
        self.rows[y as usize][x as usize] = c;
    }

    pub fn width(&self) -> isize {
        match self.rows.get(0) {
            Some(row) => row.len() as isize,
            None => 0,
        }
    }

    pub fn height(&self) -> isize {
        self.rows.len() as isize
    }

    pub fn gather_rolls(&mut self) -> u64 {
        let mut gathered_rolls: u64 = 0;
        let mut rolls_to_gather: Vec<(isize, isize)> = Vec::new();
        for x in 0..self.width() {
            for y in 0..self.height() {
                match self.get_cell(x, y) {
                    Some(c) => match c {
                        '@' => {
                            let mut roll_neighbors = 0;
                            for yoffset in -1..=1 {
                                for xoffset in -1..=1 {
                                    if yoffset == 0 && xoffset == 0 {
                                        continue;
                                    }
                                    match self.get_cell(x + xoffset, y + yoffset) {
                                        Some(oc) => {
                                            if oc == '@' {
                                                roll_neighbors += 1
                                            }
                                        }
                                        None => {}
                                    }
                                }
                            }
                            if roll_neighbors < 4 {
                                gathered_rolls += 1;
                                rolls_to_gather.push((x, y));
                            }
                        }
                        _ => {}
                    },
                    None => {}
                }
            }
        }
        for roll in rolls_to_gather {
            self.set_cell(roll.0, roll.1, 'X');
        }
        return gathered_rolls;
    }
}

impl PuzzleResult {
    pub fn process_input(input: String) -> PuzzleResult {
        let mut result = PuzzleResult {
            accessible_rolls: 0,
            recursive_accessible_rolls: 0,
        };

        let mut grid = Grid { rows: Vec::new() };

        for line in input.lines() {
            grid.add_row(line.chars().collect());
        }

        // part 1
        result.accessible_rolls = grid.gather_rolls();
        result.recursive_accessible_rolls = result.accessible_rolls;

        //part 2
        let mut returned_rolls: u64 = result.accessible_rolls;
        while returned_rolls > 0 {
            returned_rolls = grid.gather_rolls();
            result.recursive_accessible_rolls += returned_rolls;
        }
        return result;
    }
}
