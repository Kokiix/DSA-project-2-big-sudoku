pub struct Node {
    up: u32,
    down: u32,
    right: u32,
    left: u32,
    header: u32,
    row: u8,
    col: u8
}

pub struct SolvingMatrix {
    sudoku_size: usize,
    matrix: Vec<Node>,
}

impl SolvingMatrix {
    pub fn new(size: SquareNumber) -> Self {
        SolvingMatrix {
            sudoku_size: size.value,
            matrix: SolvingMatrix::init_solution_space(size.value)
        }
    }

    fn init_solution_space(n: usize) -> Vec<Node> {
        // rows = (all the ways the n values fit into n ^2 grid) = n^3
        // cols = 4 constraint types, existence, row, col, subgrid, each w n^2
        //  possible values, but we only store the 1 entry in each that corresponds
        //  to a given row
        let mut matrix = Vec::with_capacity(n.pow(3) * 4);
        return matrix;
    }
}

pub struct SquareNumber {value: usize}
impl TryFrom<u8> for SquareNumber {
    type Error = ();
    fn try_from(n: u8) -> Result<Self, Self::Error> {
        if n.isqrt().pow(2) == n  {
            Ok(SquareNumber { value: n as usize })
        } else {Err(())}
    }
}