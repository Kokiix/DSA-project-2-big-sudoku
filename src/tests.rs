use crate::solve_matrix::{SolvingMatrix, SquareNumber};

#[test]
fn create_matrix() {
    let a = SolvingMatrix::new(SquareNumber::try_from(9).unwrap());
}
