use crate::{generate_sudoku, solve_matrix::Solver};

#[test]
fn print_solved() {
    let size = 9;
    let grid = generate_sudoku(size as u32, 12346).unwrap().solved_grid;
    for (i, n) in grid.iter().enumerate() {
        if i % size == 0 {
            print!("\n");
        }
        print!("{n} ");
    }
    print!("\n");
}

#[test]
fn random() {
    Solver::solve(5, 3);
}
