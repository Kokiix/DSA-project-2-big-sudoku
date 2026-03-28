pub struct Sudoku{
    pub n: usize,
    pub subgrid: usize,
    pub board: Vec<Vec<u8>>,
}

impl Sudoku{
    pub fn new(n: usize) -> Self{
        let subgrid = (n as f64).sqrt().round() as usize;
        Sudoku{
            n,
            subgrid,
            board: vec![vec![0; n]; n],
        }
    }
}