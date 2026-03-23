use std::u32;

// TODO: prove these statements with math
// u16 is too small; max square sudoku size is ~144
#[derive(Clone)]
pub struct Node {
    pub column_obj: u32,
    pub up: u32,
    pub down: u32,
    pub right: u32,
    pub left: u32,
    pub column_size: Option<u32>,
}

impl Node {
    fn create_connected(matrix: &mut Vec<Node>, pos: u32, col_obj: u32) -> Self {
        let bot_col_node = matrix[col_obj as usize].up;
        matrix[bot_col_node as usize].down = pos;
        matrix[col_obj as usize].up = pos;
        Node {
            column_obj: col_obj,
            up: bot_col_node,
            down: col_obj,
            right: pos + 1,
            left: pos - 1,
            column_size: None,
        }
    }

    pub fn insert_row(matrix: &mut Vec<Node>, starting_pos: u32, col_positions: &[u32]) {
        for (i, col_pos) in col_positions.iter().enumerate() {
            let n = Node::create_connected(matrix, starting_pos + i as u32, *col_pos);
            matrix.push(n);
        }

        matrix[starting_pos as usize].left = starting_pos + 3;
        matrix[starting_pos as usize + 3].right = starting_pos;
    }
}

pub struct Solver {
    matrix: Vec<Node>,
    solution: Vec<usize>,
    n: u32,
    n2: u32,
    root: usize,
}

impl Solver {
    pub fn solve(n: u32) -> Vec<usize> {
        let mut s = Self::init_grid(n);
        s.explore_min_col();
        s.solution
    }

    // false = fail on this recursive branch, true = matrix solved
    fn explore_min_col(&mut self) -> bool {
        let root = self.root;
        // check if solved already
        if self.matrix[root].right == root as u32 {
            return true;
        }
        // get col w least elements
        let mut col_idx: usize = self.matrix[root].right as usize;
        let mut col_traverse = col_idx;
        let mut min_size = u32::MAX;
        while col_traverse != root {
            if let Some(size) = self.matrix[col_traverse].column_size
                && size < min_size
            {
                col_idx = col_traverse;
                min_size = size;
            }
            col_traverse = self.matrix[col_traverse].right as usize;
        }

        let col_node = self.matrix[col_idx].clone();
        let mut row_item: usize = col_node.down as usize;
        if row_item == col_idx {
            return false;
        }

        self.matrix[col_node.left as usize].right = col_node.right;
        self.matrix[col_node.right as usize].left = col_node.left;
        // try out each row in col
        while row_item != col_idx {
            // cover stuff
            self.solution.push(row_item);
            let mut row_subitem: usize = self.matrix[row_item].right as usize;
            while row_subitem != row_item {
                self.cover_row_subcols(row_subitem);
                row_subitem = self.matrix[row_subitem].right as usize;
            }

            // continue exploring
            if self.explore_min_col() {
                // leave primary column unlinked still
                return true;
            }
            // else else uncover
            self.solution.pop();
            // make sure to go in reverse order
            row_subitem = self.matrix[row_subitem].left as usize;
            while row_subitem != row_item {
                self.uncover_col(self.matrix[row_subitem].column_obj);
                row_subitem = self.matrix[row_subitem].left as usize;
            }

            row_item = self.matrix[row_item].down as usize;
        }

        self.matrix[col_node.left as usize].right = col_idx as u32;
        self.matrix[col_node.right as usize].left = col_idx as u32;

        return false;
    }

    // given a row item in col, covers col and OTHER rows in col
    fn cover_row_subcols(&mut self, start_pos: usize) {
        self.cover_col(start_pos);

        let mut col_pos: usize = self.matrix[start_pos].down as usize;
        while col_pos != start_pos {
            if self.matrix[col_pos].column_size.is_none() {
                self.cover_row(col_pos);
            }
            col_pos = self.matrix[col_pos].down as usize;
        }
    }

    fn cover_col(&mut self, node_idx: usize) {
        let col = self.matrix[self.matrix[node_idx].column_obj as usize].clone();
        self.matrix[col.left as usize].right = col.right;
        self.matrix[col.right as usize].left = col.left;
    }

    fn cover_row(&mut self, start_pos: usize) {}
    fn uncover_col(&mut self, node_pos: u32) {}

    // n must be a square number
    fn init_grid(n: u32) -> Self {
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
        for col_idx in 0..=root_idx {
            matrix.push(Node {
                column_obj: col_idx,
                up: col_idx,
                down: col_idx,
                right: col_idx + 1,
                left: col_idx - 1,
                column_size: Some(n),
            });
        }

        // wrap edges
        matrix[0].left = root_idx;
        matrix[root_idx as usize].right = 0;
        matrix[root_idx as usize].column_size = None;

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

        let solution: Vec<usize> = Vec::with_capacity(n2 as usize);
        Solver {
            matrix,
            solution,
            n,
            n2,
            root: root_idx as usize,
        }
    }
}
