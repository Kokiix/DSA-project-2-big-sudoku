mod solve_matrix;
mod tests;

use wasm_bindgen::prelude::wasm_bindgen;

use crate::solve_matrix::Node;

// TODO: implement methods here so that JS doesn't have to extract?
#[wasm_bindgen]
pub struct FinalSudokuBoard {
    // has 0s in place of empty cells
    init_grid: Vec<usize>,
    // contains 0s except for empty cells
    solved_grid: Vec<usize>,
}

#[wasm_bindgen]
pub fn generate_sudoku(n: usize) -> Option<FinalSudokuBoard> {
    // TODO: set/enforce max size?
    if n.isqrt().pow(2) != n {
        return None;
    }
    let mut matrix = init_solution_space(n);

    let n2 = n.pow(2);
    let root = 4 * n2 as usize;
    while (matrix[root].right) {}

    Some(FinalSudokuBoard {
        init_grid: vec![1; 81],
        solved_grid: vec![0; 81],
    })
}

fn init_solution_space(n: u32) -> Vec<Node> {
    // rows = (all the ways the n values fit into n ^2 grid) = n^3, + 1 for headers
    // cols = 4 constraint types, existence, row, col, subgrid, each w n^2 possible values, but each row can only match 4 total constraints
    let mut matrix = Vec::with_capacity(((n.pow(3) + 1) * 4) as usize);
    let n2 = n.pow(2);

    /* the first 4n^2 + 1 indices will be the headers, with the constraint subsections ordered as follows:
    section = starting index
    existence = 0
    row = n^2
    col = 2n^2
    subgrid = 3n^2
    root = 4n^2

    within each subsection, entries go from 1 to n in the first position, then the 2nd, etc
    */
    let root_idx = 4 * n2;
    // first col obj, linked to root
    matrix.push(Node {
        column_obj: 0,
        up: 0,
        down: 0,
        right: 1,
        left: root_idx,
        column_size: Some(0),
    });
    for col_idx in 1..root_idx {
        matrix.push(Node {
            column_obj: col_idx,
            up: col_idx,
            down: col_idx,
            right: col_idx + 1,
            left: col_idx - 1,
            column_size: Some(0),
        });
    }
    // root col obj
    matrix.push(Node {
        column_obj: root_idx,
        up: root_idx,
        down: root_idx,
        right: 0,
        left: root_idx - 1,
        column_size: Some(0),
    });

    // initialize matrix nodes
    let mut new_node_pos = root_idx + 1;
    for grid_value in 0..n {
        for grid_position in 0..n2 {
            Node::insert_row(
                &mut matrix,
                new_node_pos,
                &[
                    // pointers to the column objects corresponding to each item in the row
                    grid_position,                           // existence
                    n2 + grid_position / n + grid_value,     // row
                    2 * n2 + grid_position % n + grid_value, // col
                    3 * n2 + grid_position / n + grid_value, // subgrid
                ],
            );
            new_node_pos += 4;
        }
    }
    return matrix;
}
