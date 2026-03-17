mod solve_matrix;
mod tests;

use wasm_bindgen::prelude::wasm_bindgen;

use crate::solve_matrix::{SolvingMatrix, SquareNumber};

// TODO: implement methods here so that JS doesn't have to extract?
#[wasm_bindgen]
pub struct FinalSudokuBoard {
    // has 0s in place of empty cells
    init_grid: Vec<usize>,
    // contains 0s except for empty cells
    solved_grid: Vec<usize>,
}

#[wasm_bindgen]
pub fn generate_sudoku(input_sudoku_size: usize) -> Option<FinalSudokuBoard> {
    // TODO: choose max size allowed
    let valid_sudoku_size: SquareNumber = input_sudoku_size.try_into().ok()?;
    let mut matrix = SolvingMatrix::new(valid_sudoku_size);
    // SOLVE HERE
    Some(FinalSudokuBoard {
        init_grid: vec![1; 81],
        solved_grid: vec![0; 81],
    })
}
