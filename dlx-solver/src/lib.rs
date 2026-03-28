mod solve_matrix;
#[cfg(test)]
mod tests;

use wasm_bindgen::prelude::wasm_bindgen;

use crate::solve_matrix::{Node, Solver};

#[wasm_bindgen]
pub struct FinalSudokuBoard {
    // Has 0s in place of empty cells
    init_grid: Vec<usize>,
    solution: Vec<usize>,
    n_removed: u32,
}

#[wasm_bindgen]
impl FinalSudokuBoard {
    #[wasm_bindgen(getter)]
    pub fn init_grid(&self) -> Vec<usize> {
        self.init_grid.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn solution(&self) -> Vec<usize> {
        self.solution.clone()
    }
}

#[wasm_bindgen]
pub fn generate_sudoku(n: u32, n_empty: f32, seed: usize) -> Option<FinalSudokuBoard> {
    if n.isqrt().pow(2) != n {
        return None;
    }

    let n2 = n.pow(2) as usize;
    let (sol, removed, n_removed) = Solver::solve(n, n_empty, seed);

    // Translate row # into position + value
    let mut init_grid: Vec<usize> = vec![0; n2];
    let mut solution: Vec<usize> = vec![0; n2];
    for row_idx in sol {
        let dist_from_col = (row_idx - (4 * n2 + 1)) / 4;
        let value = dist_from_col / n2 + 1;

        init_grid[dist_from_col % n2] = if removed.contains(&row_idx) { 0 } else { value };
        solution[dist_from_col % n2] = value;
    }

    Some(FinalSudokuBoard {
        init_grid,
        solution,
        n_removed,
    })
}
