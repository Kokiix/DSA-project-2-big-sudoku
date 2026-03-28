
use crate::sudoku::Sudoku;

//Worst case O(n^(n^2)) time complexity, but with the MRV heuristic it performs much better in practice. 
//Will take too long if worst case, but should work if the puzzle is reasonably constrained.
pub struct Solver {
    n: usize,
    subgrid: usize,
    pub board: Vec<Vec<u8>>,
    row_used: Vec<u64>,
    col_used: Vec<u64>,
    box_used: Vec<u64>,
}
impl Solver {
    //n <= 36 which allows us to use a u64 to track used numbers in rows, columns, and boxes. Each bit represents whether a number is used.
    pub fn new(sudoku: Sudoku) -> Self {
        let n = sudoku.n;
        let subgrid = sudoku.subgrid;
        let board = sudoku.board;
        let mut solver =Solver{
            n,
            subgrid,
            board,
            row_used: vec![0u64;n],
            col_used: vec![0u64;n],
            box_used: vec![0u64; subgrid * subgrid],
        };
        solver.int_constraints();
        solver
    }
    /*  
        Initializes the row_used, col_used, and box_used vectors based on the initial state of the board. 
        Each bit in the vectors represents whether a number (1-n) is already used in the row, column, or box.
        This approach can be found here: https://www.geeksforgeeks.org/competitive-programming/solving-sudoku-using-bitwise-algorithm 
    */
    fn int_constraints(&mut self){
        for r in 0..self.n {
            for c in 0..self.n{
                let num = self.board[r][c];
                if num != 0 {
                    let val = 1u64 << (num - 1);
                    self.row_used[r] |= val;
                    self.col_used[c] |= val;
                    let box_index = self.getBoxIndex(r, c);
                    self.box_used[box_index] |= val;
                }
            }
        }
    }
    fn getBoxIndex(&self, row: usize, col: usize) -> usize {
        (row / self.subgrid) * self.subgrid + (col / self.subgrid)
    }
    fn valid_place(&self, row: usize, col: usize, num: u8) -> bool {
        let val = 1u64 << (num - 1);
        let box_index = self.getBoxIndex(row, col);
        (self.row_used[row] & val) == 0 && (self.col_used[col] & val) == 0 && (self.box_used[box_index] & val) == 0
    }
    fn place(&mut self, row: usize, col: usize, num: u8) {
        self.board[row][col] = num;
        let val = 1u64 << (num - 1);
        self.row_used[row] |= val;
        self.col_used[col] |= val;
        let box_index = self.getBoxIndex(row, col);
        self.box_used[box_index] |= val;
    }
    fn remove(&mut self, row: usize, col: usize, num: u8) {
        self.board[row][col] = 0;
        let val = 1u64 << (num - 1);
        self.row_used[row] &= !val;
        self.col_used[col] &= !val;
        let box_index = self.getBoxIndex(row, col); 
        self.box_used[box_index] &= !val;
    }
    fn count_paths(&self, row: usize, col: usize) -> usize {
        if self.board[row][col] != 0 {
            return 0;
        }
        let mut count = 0;
        for num in 1..=self.n {
            if self.valid_place(row, col, num as u8){
                count += 1;
            }
        }
        count
    }
    // Finds the next empty cell with the fewest valid placements using minimum remaining values(MRV) heuristic.
    // https://inst.eecs.berkeley.edu/~cs188/textbook/csp/ordering.html

    fn next_empty(&mut self) -> (bool, usize, usize) {
        let mut min_count = self.n + 1;
        let mut best_row = 0;
        let mut best_col = 0;
        let mut found = false;
        for r in 0..self.n{
            for c in 0..self.n {
                if self.board[r][c] == 0 {
                    let count = self.count_paths(r,c);
                    if count < min_count {
                        min_count = count;
                        best_row = r;
                        best_col = c;
                        found = true;
                        if count == 0 {
                            return (true, best_row, best_col);
                        }
                    }
                }
            }
        }
        (found, best_row, best_col)
    }
    fn fill_cell(&mut self) -> bool {
        let mut change = true;
        while change {
            change = false;
            for r in 0..self.n {
                for c in 0..self.n{
                    if self.board[r][c] == 0 {
                        let count = self.count_paths(r,c);
                        if count == 1{
                            for num in 1..=self.n {
                                if self.valid_place(r, c, num as u8){
                                    self.place(r, c, num as u8);
                                    break;
                                }
                            }
                            change = true;
                        }else if count == 0 {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }

    pub fn solve(&mut self) -> bool {
        self.backtrack()
    }
/* 
    We recursively try to fill the empty cells with numbers from 1 to n. 
    For every unassigned cell, we place a number and then check whether it 
    is valid to place that number in the given row, column, and subgrid. If 
    it is valid, we move to the next cell; if not, we backtrack and try another 
    number. This process continues until all cells are filled or no solution exists.
    https://www.geeksforgeeks.org/dsa/sudoku-backtracking-7/
*/
    fn backtrack(&mut self) -> bool {
        let (found, row, col) = self.next_empty();
        if !found {
            return true;
        }
        for num in 1..=self.n{
            if self.valid_place(row, col, num as u8){
                self.place(row, col, num as u8);
                if !self.fill_cell() {
                    self.remove(row, col, num as u8);
                    continue;
                }
                if self.backtrack(){
                    return true;
                }
                self.remove(row, col, num as u8);
            }
        }
        false
    }
}
