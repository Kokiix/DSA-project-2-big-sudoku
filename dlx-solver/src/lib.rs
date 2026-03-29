mod solve_matrix;
#[cfg(test)]
mod tests;

use wasm_bindgen::prelude::wasm_bindgen;

use crate::solve_matrix::Solver;

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
/// n must be a square number
pub fn generate_sudoku(n: u32, proportion_empty: f32, seed: usize) -> FinalSudokuBoard {
    let (sol, removed, n_removed) = Solver::generate(n, proportion_empty, seed);
    let n2 = n.pow(2) as usize;

    // Translate row # into position + value
    let solution = matrix_sol_into_sudoku_board(sol.clone(), n2);
    let init_rows = sol.into_iter().filter(|r| !removed.contains(r)).collect();
    let init_grid = matrix_sol_into_sudoku_board(init_rows, n2);

    FinalSudokuBoard {
        init_grid,
        solution,
        n_removed,
    }
}

#[wasm_bindgen]
pub fn solve_sudoku(n: u32, in_board: Vec<usize>) -> Vec<usize> {
    let mut s = Solver::init_matrix(n);
    let n2 = n.pow(2) as usize;

    for (pos, &val) in in_board.iter().enumerate() {
        if val != 0 {
            let row_idx = (4 * n2 + 1) + 4 * ((val - 1) * n2 + pos);
            s.insert_sol_row(row_idx);
        }
    }

    s.first_sol_found = false;
    s.n_solutions = 0;
    s.find_solution_branch();
    matrix_sol_into_sudoku_board(s.get_sol(), n2)
}

fn matrix_sol_into_sudoku_board(rows: Vec<usize>, n2: usize) -> Vec<usize> {
    let mut solution = vec![0; n2];
    for row_idx in rows {
        let dist_from_col = (row_idx - (4 * n2 + 1)) / 4;
        let value = dist_from_col / n2 + 1;
        solution[dist_from_col % n2] = value;
    }
    return solution;
}
