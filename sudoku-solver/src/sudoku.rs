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

    pub fn does_num_exist(&self, row: i32, col: i32, num: i32) -> bool {
        // checking column
        // for temp_row in 0..9 {
        //     if self.board[temp_row][col as usize] == num {
        //         return true;
        //     }
        // }

        for temp_row in self.board {
            if temp_row[col as usize] == num {
                return true;
            }
        }

        // checking row
        // for temp_col in 0..9 {
        //     if self.board[row as usize][temp_col] == num {
        //         return true;
        //     }
        // }

        for value in &self.board[row as usize] {
            if *value == num {
                return true;
            }
        }

        // check the grid the number is in
        let box_start_row: usize = (row - row % 3) as usize;
        let box_start_col: usize = (col - col % 3) as usize;

        for temp_row in 0..3 {
            for temp_col in 0..3 {
                if self.board[box_start_row + temp_row][box_start_col + temp_col] == num {
                    return true;
                }
            }
        }

        false
    }

    pub fn is_num_valid(&self, row: i32, col: i32, num: i32) -> bool {
        !self.does_num_exist(row, col, num)
    }

    pub fn does_empty_exist(&self, row: &mut i32, col: &mut i32) -> bool {
        for i in 0..9 {
            for j in 0..9 {
                if self.board[i][j] == 0 {
                    *row = i as i32;
                    *col = j as i32;
                    return true;
                }
            }
        }

        false
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
        // println!("Loaded sudoku from file: {}", filename);
        // for row in &self.board {
        //     for &num in row {
        //         print!("{} ", num);
        //     }
        //     println!();
        // }

        Ok(()) // Explicitly return an 'Ok' variant to indicate success
    }

    pub fn solve(&mut self) -> bool {
        let mut row: i32 = 0;
        let mut col: i32 = 0;

        if !self.does_empty_exist(&mut row, &mut col) {
            return true;
        }
        for num in 1..10 {
            if self.is_num_valid(row, col, num) {
                self.board[row as usize][col as usize] = num;
                if self.solve() {
                    return true;
                }
                self.board[row as usize][col as usize] = 0;
            }
        }

        false
    }

    pub fn print(&self) {
        for row in &self.board {
            for &num in row {
                print!("{} ", num);
            }
            println!();
        }
    }
}
