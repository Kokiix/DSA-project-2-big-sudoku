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
        // get col w least elements
        let mut col_idx: usize = self.matrix[root].right as usize;
        let mut min_size = u32::MAX;
        while col_idx != root {
            min_size = std::cmp::min(self.matrix[col_idx].column_size.unwrap(), min_size);
            col_idx = self.matrix[col_idx].right as usize;
        }
        col_idx = min_size as usize;

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
                self.cover_col(self.matrix[row_subitem].column_obj);
                row_subitem = self.matrix[row_subitem].right as usize;
            }
            // if the cover worked then solution is found
            if self.matrix[root].right == root as u32 {
                return true;
            }
            // else keep exploring
            if self.explore_min_col() {
                return true;
            }
            // else else uncover
            self.solution.pop();
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

    fn cover_col(&mut self, col_idx: u32) {}
    fn uncover_col(&mut self, col_idx: u32) {}

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
        matrix.push(Node {
            column_obj: 0,
            up: 0,
            down: 0,
            right: 1,
            left: root_idx,
            column_size: Some(9),
        });
        for col_idx in 1..root_idx {
            matrix.push(Node {
                column_obj: col_idx,
                up: col_idx,
                down: col_idx,
                right: col_idx + 1,
                left: col_idx - 1,
                column_size: Some(9),
            });
        }
        // root col obj
        matrix.push(Node {
            column_obj: root_idx,
            up: root_idx,
            down: root_idx,
            right: 0,
            left: root_idx - 1,
            column_size: Some(9),
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
