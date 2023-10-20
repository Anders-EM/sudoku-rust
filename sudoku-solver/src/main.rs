mod sudoku;

fn main() {
    println!("Hello, world!");
    let mut sudoku = sudoku::Sudoku::new();
    sudoku.load_from_file(String::from(
        "/Users/kurt/Desktop/GHProjs/sudoku-rust/txt/sudoku-test1.txt",
    ));
}
