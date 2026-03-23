mod solve_matrix;
mod tests;

use wasm_bindgen::prelude::wasm_bindgen;

use crate::solve_matrix::{Node, Solver};

// TODO: implement methods here so that JS doesn't have to extract?
#[wasm_bindgen]
pub struct FinalSudokuBoard {
    // has 0s in place of empty cells
    init_grid: Vec<usize>,
    // contains 0s except for empty cells
    solved_grid: Vec<usize>,
}

#[wasm_bindgen]
pub fn generate_sudoku(n: u32) -> Option<FinalSudokuBoard> {
    // TODO: set/enforce max size?
    if n.isqrt().pow(2) != n {
        return None;
    }
    let sol: Vec<usize> = Solver::solve(n);

    Some(FinalSudokuBoard {
        init_grid: vec![1; 81],
        solved_grid: vec![0; 81],
    })
}
