mod solve_matrix;
mod tests;

use wasm_bindgen::prelude::wasm_bindgen;

use crate::solve_matrix::{Node, Solver};

// TODO: implement methods here so that JS doesn't have to extract?
#[wasm_bindgen]
pub struct FinalSudokuBoard {
    // Has 0s in place of empty cells
    init_grid: Vec<usize>,
    solution: Vec<usize>,
}

#[wasm_bindgen]
pub fn generate_sudoku(n: u32, n_empty: u32, seed: usize) -> Option<FinalSudokuBoard> {
    if n.isqrt().pow(2) != n {
        return None;
    }

    let n2 = n.pow(2) as usize;
    let (sol, removed) = Solver::solve(n, n_empty, seed);

    let mut init_grid: Vec<usize> = vec![0; n2];
    let mut solution: Vec<usize> = vec![0; n2];

    // Translate matrix row # into position + value
    for row_idx in sol {
        let dist_from_col = (row_idx - (4 * n2 + 1)) / 4;
        let value = dist_from_col / n2 + 1;

        init_grid[dist_from_col % n2] = if removed.contains(&row_idx) { 0 } else { value };
        solution[dist_from_col % n2] = value;
    }

    Some(FinalSudokuBoard {
        init_grid,
        solution,
    })
}
