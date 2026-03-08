use wasm_bindgen::prelude::wasm_bindgen;

// TODO: implement methods here so that JS doesn't have to extract?
#[wasm_bindgen]
pub struct FinalSudokuBoard {
    init_grid: Vec<u16>,
    solved_grid: Vec<u16>,
}

#[wasm_bindgen]
pub fn generate_sudoku(grid_size: u16) -> FinalSudokuBoard {
    return FinalSudokuBoard { init_grid: vec![], solved_grid: vec![] };
}