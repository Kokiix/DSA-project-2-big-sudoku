mod solve_matrix;
mod tests;

use wasm_bindgen::prelude::wasm_bindgen;

use crate::solve_matrix::{Node, Solver};

// TODO: implement methods here so that JS doesn't have to extract?
#[wasm_bindgen]
pub struct FinalSudokuBoard {
    // Has 0s in place of empty cells
    init_grid: Vec<usize>,
    // Contains 0s except for empty cells
    solved_grid: Vec<usize>,
}

#[wasm_bindgen]
pub fn generate_sudoku(n: u32, seed: usize) -> Option<FinalSudokuBoard> {
    if n.isqrt().pow(2) != n {
        return None;
    }

    let n2 = n.pow(2) as usize;
    let sol: Vec<usize> = Solver::solve(n, seed);
    let mut solved: Vec<usize> = vec![0; n2];

    for row_idx in sol {
        let row_idx = (row_idx - (4 * n2 + 1)) / 4;
        solved[row_idx % n2] = row_idx / n2 + 1;
    }

    Some(FinalSudokuBoard {
        init_grid: vec![1; 81],
        solved_grid: solved,
    })
}
