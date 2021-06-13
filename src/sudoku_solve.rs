use std::char;

pub struct Sudoku {
    pub n: usize,
    pub board: Vec<Vec<char>>,
}

//helper fxn to display board
impl Sudoku {
    pub fn display(&self) {
        for i in 0..self.n {
            for j in 0..self.n {
                print!("{} ", self.board[i][j]);
            }
            println!();
        }
    }

    //helper fxn to check if [i,j] is filled can be filled with a particular num
    pub fn is_valid(&self, i: usize, j: usize, num: char) -> bool {
        // searching in the same row and column
        for ind in 0..self.n {
            if self.board[i][ind] == num || self.board[ind][j] == num {
                return false;
            }
        }
        //searching in the 3x3 grid
        let sub_row = (i / 3) * 3;
        let sub_col = (j / 3) * 3;
        for i in sub_row..sub_row + 3 {
            for j in sub_col..sub_col + 3 {
                if self.board[i][j] == num {
                    return false;
                }
            }
        }
        return true;
    }

    // & mut as board should be editable therefore mutable borrow
    pub fn backtrack(&mut self, i: usize, j: usize) -> bool {
        // base case where last row is completed
        if i == self.n {
            return true;
        }
        // case where one row is completed
        if j == self.n {
            return self.backtrack(i + 1, 0);
        }
        //case where we have to progress the bactracking to find best number for next place
        if self.board[i][j] != '.' {
            return self.backtrack(i, j + 1);
        }
        // normal case which should be handled
        for candidate in 1..10 {
            let c: char = char::from_digit(candidate, 10).unwrap();
            if self.is_valid(i, j, c) {
                // check if we place the number is the whole board solvable if yes then return true
                self.board[i][j] = c;
                if self.backtrack(i, j + 1) {
                    return true;
                }
                self.board[i][j] = '.';
            }
        }
        return false;
    }

    pub fn solve_board(&mut self) -> bool {
        return self.backtrack(0, 0);
    }
}
