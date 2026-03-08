mod solve_matrix;
mod tests;

use wasm_bindgen::prelude::wasm_bindgen;

use crate::solve_matrix::SolvingMatrix;

// TODO: implement methods here so that JS doesn't have to extract?
#[wasm_bindgen]
pub struct FinalSudokuBoard {
    init_grid: Vec<u16>,
    solved_grid: Vec<u16>,
}

#[wasm_bindgen]
pub fn generate_sudoku(grid_size: usize) -> FinalSudokuBoard {
    let mut matrix = SolvingMatrix::new(grid_size);
    return FinalSudokuBoard { init_grid: vec![], solved_grid: vec![] };
}