use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub struct Sudoku {
    pub board: [[i32; 9]; 9],
}

impl Sudoku {
    pub fn new() -> Sudoku {
        Sudoku { board: [[0; 9]; 9] }
    }

    pub fn equals(&self, other: &Sudoku) -> bool {
        for i in 0..9 {
            for j in 0..9 {
                if self.board[i][j] != other.board[i][j] {
                    return false;
                }
            }
        }

        true
    }

    pub fn load_from_file(&mut self, filename: String) -> io::Result<()> {
        let file = File::open(&filename)?;
        let reader = BufReader::new(file);

        for (i, line) in reader.lines().enumerate() {
            let line = line?;
            let values: Vec<i32> = line
                .split_whitespace()
                .filter_map(|num| num.parse().ok())
                .collect();

            for (j, &value) in values.iter().enumerate() {
                self.board[i][j] = value;
            }
        }

        // Fixed print statement
        println!("Loaded sudoku from file: {}", filename);
        for row in &self.board {
            for &num in row {
                print!("{} ", num);
            }
            println!();
        }

        Ok(()) // Explicitly return an 'Ok' variant to indicate success
    }
}
