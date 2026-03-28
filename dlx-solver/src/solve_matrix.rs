/*!
 * Create a circular doubly linked list to represent all states of the given board size, then recursively traverse the list to solve the matrix.
 *
 * Returns a list of node indices, to be decoded using the below, though the implementation should only ever live in lib.rs
 *
 * # Matrix Structure
 * ## In Theory:
 * Rows: (all the ways the n values fit into n^2 grid) = n * n^2 = n^3, + 1 row for column objects
 * Columns: Each column falls into 1 of 4 constraint types: existence, row, col, subgrid.
 * For example, there could be a column that represents the constraint that a solution must have some number in (0, 0).
 * Therefore, each column has n values, but only 1 may be present in any given solution.
 *
 * ## In Implementation:
 * The rows are laid out end to end in a Vec of size 4(n^3 + 1).
 * Each node will "point" to another node by containing the index of the other node, rather than a memory location.
 * The first 4n^2 + 1 indices will be the headers, with the 4 constraint types beginning at the following indices.
 *      existence = 0
 *      row = n^2
 *      col = 2n^2
 *      subgrid = 3n^2
 *      root = 4n^2
 *
 *
 * The actual nodes themselves are ordered by position, then value.
 * Ex: (1 in (0, 0)), (1 in (1, 0))...(2 in (0, 0)), (2 in (1, 0))...etc
!*/

use wasm_bindgen::prelude::wasm_bindgen;

// u16 is too small; max square sudoku size is ~144
// TODO: prove the above in a comment?
#[derive(Clone)]
struct Node {
    column_obj: u32,
    up: u32,
    down: u32,
    right: u32,
    left: u32,
    column_size: Option<u32>,
}

#[derive(Clone)]
#[wasm_bindgen]
pub struct Solver {
    blank_matrix: Vec<Node>,
    matrix: Vec<Node>,
    #[wasm_bindgen(getter_with_clone)]
    pub solution: Vec<usize>,
    #[wasm_bindgen(getter_with_clone)]
    pub removed: Vec<usize>,
    root: usize,
    pub n: u32,
    rng_state: u32,

    initialized: bool,
    n_solutions: usize,
}

impl Solver {
    /// The randomization is based on 32 bit xorshift, so seed must be 32 bit
    pub fn new(n: u32, seed: usize) -> Solver {
        let mut s = Self::init_matrix(n); // n MUST be a square number, crashes otherwise...
        s.blank_matrix = s.matrix.clone();
        s.rng_state = if seed == 0 { 1 } else { seed as u32 };
        s.find_solution_branch();
        s.initialized = true;
        return s;
    }

    fn init_matrix(n: u32) -> Self {
        let mut matrix = Vec::with_capacity(((n.pow(3) + 1) * 4) as usize);
        let n2 = n.pow(2);
        let root_idx = 4 * n2;

        // Init column objects
        // (The 1st node can't go in loop bc. left "pointer" would go negative; has to wrap around to root on right side)
        matrix.push(Node {
            column_obj: 0,
            up: 0,
            down: 0,
            right: 1,
            left: root_idx,
            column_size: Some(n),
        });

        for col_idx in 1..=root_idx {
            matrix.push(Node {
                column_obj: col_idx,
                up: col_idx,
                down: col_idx,
                right: col_idx + 1,
                left: col_idx - 1,
                column_size: Some(n),
            });
        }
        matrix[root_idx as usize].right = 0;
        matrix[root_idx as usize].column_size = None;

        // Init the 4n^3 nodes, 1 row at a time.
        let mut insert_idx = root_idx + 1;
        let sqrt_n = (n as f64).sqrt() as u32;

        for grid_value in 0..n {
            for grid_position in 0..n2 {
                let row = grid_position / n;
                let col = grid_position % n;
                let subgrid = (row / sqrt_n) * sqrt_n + (col / sqrt_n);

                Self::insert_row(
                    &mut matrix,
                    insert_idx,
                    &[
                        // Positions of the columns that each node should fall into
                        grid_position,                     // existence
                        n2 + row * n + grid_value,         // row
                        2 * n2 + col * n + grid_value,     // col
                        3 * n2 + subgrid * n + grid_value, // subgrid
                    ],
                );

                insert_idx += 4;
            }
        }

        let solution: Vec<usize> = Vec::with_capacity(n2 as usize);
        Solver {
            blank_matrix: matrix.clone(),
            matrix,

            solution: solution.clone(),
            removed: solution,

            root: root_idx as usize,
            n,
            rng_state: 0,
            initialized: false,
            n_solutions: 0,
        }
    }

    /// Recursively search for solution. Returning false = fail on this recursive branch, true = matrix solved, short circuiting
    fn find_solution_branch(&mut self) -> bool {
        let root = self.root;
        if self.matrix[root].right == root as u32 {
            self.n_solutions += 1;
            return true;
        }

        // Choose col w least elements as heuristic, hopefully reducing search time
        let mut col_obj: usize = self.matrix[root].right as usize;
        let mut col_traverse = col_obj;
        let mut min_size = self.n;
        while col_traverse != root {
            if let Some(size) = self.matrix[col_traverse].column_size {
                if size == 0 {
                    return false;
                }
                if size < min_size {
                    col_obj = col_traverse;
                    min_size = size;
                    if min_size <= 1 {
                        break;
                    }
                }
            }
            col_traverse = self.matrix[col_traverse].right as usize;
        }

        // Randomize first col chosen (surely this won't trigger after first and hit a hidden one)
        if min_size == self.n {
            col_obj = self.rand_int_to(self.root);
        }

        let col_node = self.matrix[col_obj].clone();
        let mut row_item: usize = col_node.down as usize;
        if col_node.column_size.unwrap() == 0 {
            return false;
        }

        self.cover_col_and_rows(col_obj);

        // Grab all rows (potential solution branches)
        let mut rows = Vec::with_capacity(col_node.column_size.unwrap() as usize);
        while row_item != col_obj {
            rows.push(row_item);
            row_item = self.matrix[row_item].down as usize;
        }
        // Shuffle rows (Fisher-Yates)
        for i in (1..rows.len()).rev() {
            let j = self.rand_int_to(i + 1);
            rows.swap(i, j);
        }

        // Then try to recurse further for each row
        for row in rows {
            self.solution.push(row);
            self.cover_solution(row);

            // Continue recursion
            if self.find_solution_branch() && !self.initialized {
                return true;
            }

            // Reverse of self.cover_solution();
            self.solution.pop();
            let mut row_subitem = self.matrix[row].left as usize;
            while row_subitem != row {
                let j_col = self.matrix[row_subitem].column_obj as usize;
                self.uncover_col_and_rows(j_col);
                row_subitem = self.matrix[row_subitem].left as usize;
            }

            if self.n_solutions > 1 {
                break;
            }
        }

        self.uncover_col_and_rows(col_obj);

        return self.n_solutions == 1;
    }

    pub fn remove_cells(&mut self, n_to_remove_proportion: f32) -> u32 {
        let mut n_removed = 0;
        let mut to_remove = self.solution.clone();

        let mut cell_removed = true;
        let n_to_remove = (n_to_remove_proportion * self.n.pow(2) as f32) as u32;
        while n_removed < n_to_remove && cell_removed {
            cell_removed = false;
            let mut next: Vec<usize> = Vec::new();
            for cell_to_be_removed in &to_remove {
                self.matrix = self.blank_matrix.clone();
                for cell in self.solution.clone() {
                    if cell != *cell_to_be_removed && !self.removed.contains(&cell) {
                        self.cover_col_and_rows(self.matrix[cell].column_obj as usize);
                        self.cover_solution(cell);
                    }
                }

                self.n_solutions = 0;
                if self.find_solution_branch() {
                    n_removed += 1;
                    self.removed.push(*cell_to_be_removed);
                    cell_removed = true;
                } else {
                    next.push(*cell_to_be_removed);
                }

                if n_removed == n_to_remove {
                    break;
                }
            }
            to_remove = next;
        }

        return n_removed;
    }
}

// Auxillary methods
impl Solver {
    fn cover_solution(&mut self, row: usize) {
        let mut row_subitem: usize = self.matrix[row].right as usize;
        // Cover cols that this row satisfies, and eliminate some overlapping rows / answers
        while row_subitem != row {
            let j_col = self.matrix[row_subitem].column_obj as usize;
            self.cover_col_and_rows(j_col);
            row_subitem = self.matrix[row_subitem].right as usize;
        }
    }

    fn cover_col_and_rows(&mut self, start_pos: usize) {
        let col = self.matrix[self.matrix[start_pos].column_obj as usize].clone();
        self.matrix[col.left as usize].right = col.right;
        self.matrix[col.right as usize].left = col.left;

        let mut row: usize = self.matrix[start_pos].down as usize;
        while row != start_pos {
            if self.matrix[row].column_size.is_none() {
                self.cover_row(row);
            }
            row = self.matrix[row].down as usize;
        }
    }

    fn cover_row(&mut self, start_pos: usize) {
        let mut row_pos = self.matrix[start_pos].right as usize;
        let mut row_node = self.matrix[row_pos].clone();
        while row_pos != start_pos {
            self.matrix[row_node.up as usize].down = row_node.down;
            self.matrix[row_node.down as usize].up = row_node.up;
            let s = self.matrix[row_node.column_obj as usize]
                .column_size
                .unwrap();
            if s > 0 {
                self.matrix[row_node.column_obj as usize].column_size = Some(s - 1);
            }

            row_pos = self.matrix[row_pos].right as usize;
            row_node = self.matrix[row_pos].clone();
        }
    }

    // These 2 should exactly mirror the 2 functions above
    fn uncover_col_and_rows(&mut self, start_pos: usize) {
        let mut row: usize = self.matrix[start_pos].up as usize;
        while row != start_pos {
            if self.matrix[row].column_size.is_none() {
                self.uncover_row(row);
            }
            row = self.matrix[row].up as usize;
        }

        let col_idx = self.matrix[start_pos].column_obj as usize;
        let col = self.matrix[col_idx].clone();
        self.matrix[col.left as usize].right = col_idx as u32;
        self.matrix[col.right as usize].left = col_idx as u32;
    }

    fn uncover_row(&mut self, start_pos: usize) {
        let mut row_pos = self.matrix[start_pos].left as usize;
        let mut row_node = self.matrix[row_pos].clone();
        while row_pos != start_pos {
            self.matrix[row_node.up as usize].down = row_pos as u32;
            self.matrix[row_node.down as usize].up = row_pos as u32;
            let s = self.matrix[row_node.column_obj as usize]
                .column_size
                .unwrap();
            self.matrix[row_node.column_obj as usize].column_size = Some(s + 1);

            row_pos = self.matrix[row_pos].left as usize;
            row_node = self.matrix[row_pos].clone();
        }
    }

    // Helper for initializing the matrix
    fn insert_row(matrix: &mut Vec<Node>, row_start: u32, col_objs: &[u32]) {
        for (i, col_obj) in col_objs.iter().enumerate() {
            let node_pos = row_start + i as u32;
            let bot_col_node = matrix[*col_obj as usize].up;
            matrix[bot_col_node as usize].down = node_pos;
            matrix[*col_obj as usize].up = node_pos;
            matrix.push(Node {
                column_obj: *col_obj,
                up: bot_col_node,
                down: *col_obj,
                right: node_pos + 1,
                left: node_pos - 1,
                column_size: None,
            })
        }

        matrix[row_start as usize].left = row_start + 3;
        matrix[row_start as usize + 3].right = row_start;
    }

    // Integer from 0 to max
    fn rand_int_to(&mut self, non_incl_max: usize) -> usize {
        return (self.xorshift() as usize) % non_incl_max;
    }

    fn xorshift(&mut self) -> u32 {
        let mut rng = self.rng_state;
        rng ^= rng << 13;
        rng ^= rng >> 17;
        rng ^= rng << 5;
        self.rng_state = rng;
        return rng;
    }
}
