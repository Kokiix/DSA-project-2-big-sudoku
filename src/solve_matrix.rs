// TODO: prove these statements with math
// u16 is too small; max square sudoku size is ~144
pub struct Node {
    column_obj: u32,
    up: u32,
    down: u32,
    right: u32,
    left: u32,
    column_size: Option<u32>,
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
        root = 4n^2 */
        let root_idx = 4 * n2;
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
        matrix.push(Node {
            column_obj: root_idx,
            up: root_idx,
            down: root_idx,
            right: 0,
            left: root_idx - 1,
            column_size: Some(0),
        });

        // initialize matrix nodes
        for grid_value in 1..=n {
            for grid_position in 0..n2 {
                let existence = Node {};
            }
        }
        return matrix;
    }
}

pub struct SquareNumber {
    value: usize,
}
impl TryFrom<u8> for SquareNumber {
    type Error = ();
    fn try_from(n: u8) -> Result<Self, Self::Error> {
        if n.isqrt().pow(2) == n {
            Ok(SquareNumber { value: n as usize })
        } else {
            Err(())
        }
    }
}
