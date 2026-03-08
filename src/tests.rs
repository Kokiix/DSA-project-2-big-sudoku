use crate::solve_matrix::{SolvingMatrix, SquareNumber};

#[test]
fn create_matrix() {
    let _ = SolvingMatrix::new(SquareNumber::from(9).unwrap());
}