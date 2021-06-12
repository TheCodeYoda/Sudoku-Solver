use std::char;

//helper fxn to display board
pub fn display(board: &Vec<Vec<char>> , n:usize) {
    for i in 0..n {
	for j in 0..n {
	    print!("{} ",board[i][j]);
	}
	println!();
    } 
}

    
//helper fxn to check if [i,j] is filled can be filled with a particular num 
pub fn is_valid(i: usize , j: usize , n:usize , num:char, board: &Vec<Vec<char>>) -> bool {
    // searching in the same row and column
    for ind in 0..n {
	if board[i][ind] == num || board[ind][j] == num {
	    return false;
	}
    }
    //searching in the 3x3 grid
    let sub_row = (i/3) * 3;
    let sub_col = (j/3) * 3;
    for i in sub_row..sub_row+3 {
	for j in sub_col..sub_col+3 {
	    if board[i][j] == num {
		return false;
	    }
	}
    }
    return true;
}

// & mut as board should be editable therefore mutable borrow
pub fn backtrack(i: usize , j: usize , n:usize , board: &mut Vec<Vec<char>>) -> bool {
    // base case where last row is completed
    if i==n {
	return true;
    }
    // case where one row is completed
    if j==n {
	return backtrack(i+1,0,n,board);
    }
    //case where we have to progress the bactracking to find best number for next place
    if board[i][j]!='.' {
	return backtrack(i,j+1,n,board);
    }
    // normal case which should be handled
    for candidate in 1..10 {
	let c:char = char::from_digit(candidate,10).unwrap();
	if is_valid(i,j,n,c,board) {
	    // check if we place the number is the whole board solvable if yes then return true
	    board[i][j] = c;
	    if backtrack(i,j+1,n,board) {
		return true;
	    }    
	    board[i][j] = '.';
	}
    }
    return false;
}


pub fn solve_board(n: usize , board: &mut Vec<Vec<char>>) -> bool {
   return backtrack(0,0,n,board);
}
