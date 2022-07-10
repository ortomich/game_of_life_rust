
pub struct Game {
    pub width: usize,
    pub height: usize,
    pub world: Vec<Vec<u8>>,
}

impl Game {
    pub fn draw(&self) -> () {
        for line in &self.world {
            println!(
                "{}",
                line.iter()
                    .map(|l| {
                        if l == &1 {
                            return "• ".to_string();
                        } else {
                            return "  ".to_string();
                        };
                    })
                    .collect::<String>()
            );
        }
    }

    pub fn evolution(&mut self) {
        let operations: [[i8; 2]; 8] = [
            [0, 1],
            [0, -1],
            [1, 0],
            [-1, 0],
            [1, 1],
            [1, -1],
            [-1, 1],
            [-1, -1],
        ];

        let mut new = self.world.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let mut lives = 0;

                for op in operations {
                    let new_row = row as i8 + op[0];
                    let new_col = col as i8 + op[1];

                    if let Some(row) = self.world.get(new_row as usize) {
                        if let Some(cell) = row.get(new_col as usize) {
                            if cell == &1 {
                                lives += 1;
                            }
                        }
                    }
                }

                if lives == 3 || lives == 2 && self.world[row][col] == 1 {
                    new[row][col] = 1;
                } else {
                    new[row][col] = 0;
                };
            }
        }

        self.world = new;
    }
}