mod sudoku;

fn main() {
    println!("Hello, world!");
    let mut sudoku = sudoku::Sudoku::new();
    sudoku.load_from_file(String::from(
        "/Users/kurt/Desktop/GHProjs/sudoku-rust/txt/sudoku-test1.txt",
    ));
    let mut solved = sudoku::Sudoku::new();
    solved.load_from_file(String::from(
        "/Users/kurt/Desktop/GHProjs/sudoku-rust/txt/sudoku-test1-solution.txt",
    ));

    sudoku.solve();

    print!("Solved sudoku:\n");

    sudoku.print();

    print!("{} ", sudoku.equals(&solved));

    println!("\n\n\n");

    let mut sudoku_2 = sudoku::Sudoku::new();
    sudoku_2.load_from_file(String::from(
        "/Users/kurt/Desktop/GHProjs/sudoku-rust/txt/sudoku-test2.txt",
    ));

    let mut solved_2 = sudoku::Sudoku::new();
    solved_2.load_from_file(String::from(
        "/Users/kurt/Desktop/GHProjs/sudoku-rust/txt/sudoku-test2-solution.txt",
    ));

    sudoku_2.solve();

    print!("Solved sudoku:\n");

    sudoku_2.print();

    print!("{} ", sudoku_2.equals(&solved_2));

    println!("\n\n\n");

    let mut sudoku_3 = sudoku::Sudoku::new();
    sudoku_3.load_from_file(String::from(
        "/Users/kurt/Desktop/GHProjs/sudoku-rust/txt/sudoku-impossible.txt",
    ));

    let impossible = sudoku_3.solve();
    print!("{} ", impossible);
    print!("\nimpossible sudoku\n");
}
