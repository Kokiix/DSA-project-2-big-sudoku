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
impl TryFrom<u16> for SquareNumber {
    type Error = ();
    fn try_from(n: u16) -> Result<Self, Self::Error> {
        if n.isqrt().pow(2) == n  {
            Ok(SquareNumber { value: n as usize })
        } else {Err(())}
    }
}