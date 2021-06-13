mod sudoku_solve;

use text_io::read;

fn read_board(n: usize) -> Vec<Vec<char>> {
    let mut board: Vec<Vec<char>> = Vec::new();
    for _ in 0..n {
        let mut row: Vec<char> = Vec::new();
        for _ in 0..n {
            let elem: char = read!();
            row.push(elem);
        }
        board.push(row);
    }
    return board;
}

fn main() {
    println!("Enter size of grid followed by the grid....");

    let n: usize = read!();
    let board = read_board(n);

    let mut instance = sudoku_solve::Sudoku { n, board };
    println!("\nBefore solving...");
    instance.display();
    instance.solve_board();
    println!("\nAfter solving...");
    instance.display();
}
