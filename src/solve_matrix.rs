// TODO: prove these statements with math
// u16 is too small; max square sudoku size is ~144
struct Node {
    column_obj: u32,
    up: u32,
    down: u32,
    right: u32,
    left: u32,
    column_size: Option<u32>,
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

    fn insert_row(matrix: &mut Vec<Node>, starting_pos: u32, col_positions: &[u32]) {
        for (i, col_pos) in col_positions.iter().enumerate() {
            let n = Node::create_connected(matrix, starting_pos + i as u32, *col_pos);
            matrix.push(n);
        }

        matrix[starting_pos as usize].left = starting_pos + 3;
        matrix[starting_pos as usize + 3].right = starting_pos;
    }
}

pub struct SolvingMatrix {
    sudoku_size: usize,
    matrix: Vec<Node>,
}

impl SolvingMatrix {
    pub fn new(size: SquareNumber) -> Self {
        SolvingMatrix {
            sudoku_size: size.value,
            matrix: SolvingMatrix::init_solution_space(size.value as u32),
        }
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
}

pub struct SquareNumber {
    value: usize,
}
impl TryFrom<usize> for SquareNumber {
    type Error = ();
    fn try_from(n: usize) -> Result<Self, Self::Error> {
        if n.isqrt().pow(2) == n {
            Ok(SquareNumber { value: n as usize })
        } else {
            Err(())
        }
    }
}
