pub struct PuzzleResult {
    pub accessible_rolls: u64,
}

struct Grid {
    rows: Vec<Vec<char>>,
}

impl Grid {
    pub fn add_row(&mut self, row: Vec<char>) {
        self.rows.push(row);
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Option<char> {
        Some(self.rows.get(y)?.get(x)?.to_owned())
    }

    pub fn width(&self) -> usize {
        match self.rows.get(0) {
            Some(row) => row.len(),
            None => 0,
        }
    }

    pub fn height(&self) -> usize {
        self.rows.len()
    }
}

impl PuzzleResult {
    pub fn process_input(input: String) -> PuzzleResult {
        let mut result = PuzzleResult {
            accessible_rolls: 0,
        };

        let mut grid = Grid { rows: Vec::new() };

        for line in input.lines() {
            grid.add_row(line.chars().collect());
        }

        for y in 0..grid.height() {
            for x in 0..grid.width() {
                match grid.get_cell(x, y) {
                    Some(c) => match c {
                        '@' => {
                            let mut roll_neighbors = 0;
                            for yoffset in -1..=1 {
                                for xoffset in -1..=1 {
                                    if (yoffset == 0 && xoffset == 0) || yoffset < 0 || xoffset < 0
                                    {
                                        continue;
                                    }
                                    println!("{} {}", x + xoffset as usize, y + yoffset as usize);
                                    match grid.get_cell(x + xoffset as usize, y + yoffset as usize)
                                    {
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
                                result.accessible_rolls += 1;
                            }
                        }
                        _ => {}
                    },
                    None => {}
                }
            }
            println!();
        }

        return result;
    }
}
