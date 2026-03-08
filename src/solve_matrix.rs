pub struct SolvingMatrix {
    sudoku_size: usize,
    matrix: Vec<u16>,
}

impl SolvingMatrix {
    pub fn new(size: SquareNumber) -> Self {
        SolvingMatrix {
            sudoku_size: size.value,
            matrix: Vec::with_capacity(size.value.pow(2))
        }
    }
}

pub struct SquareNumber {value: usize}
impl SquareNumber {
    pub fn from(n: u16) -> Option<Self> {
        if n.isqrt().pow(2) == n  {
            Some(SquareNumber { value: n as usize })
        } else {None}
    }
}