mod SudokuSolve;

use text_io::scan;
use text_io::read;

fn main() {
    println!("Enter size of grid followed by the grid....");

    let n:usize = read!();
    let mut board: Vec<Vec<char>> = Vec::new();

    for i in 0..n {
	let mut row: Vec<char> = Vec::new();
	for j in 0..n {
	    let elem:char = read!();
	    row.push(elem);
	}
	board.push(row);
    }
    println!("\nBefore solving...");
    SudokuSolve::display(&board,n);
    SudokuSolve::solve_board(n,&mut board);
    println!("\nAfter solving...");
    SudokuSolve::display(&board,n);
	
}
