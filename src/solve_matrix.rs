pub struct SolvingMatrix {
    matrix: Vec<u16>,
}

impl SolvingMatrix {
    pub fn new(size: usize) -> Self {
        SolvingMatrix {
            matrix: Vec::with_capacity(size.pow(2))
        }
    }
}