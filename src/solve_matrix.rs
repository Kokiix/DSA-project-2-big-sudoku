// TODO: prove these statements with math
// u16 is too small; max square sudoku size is ~144
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

    fn insert_row(matrix: &mut Vec<Node>, starting_pos: u32, col_positions: &[u32]) {
        for (i, col_pos) in col_positions.iter().enumerate() {
            let n = Node::create_connected(matrix, starting_pos + i as u32, *col_pos);
            matrix.push(n);
        }

        matrix[starting_pos as usize].left = starting_pos + 3;
        matrix[starting_pos as usize + 3].right = starting_pos;
    }
}
