use crate::{generate_sudoku, solve_matrix::Solver};

#[test]
fn print_solved() {
    let size = 36;
    let sol = generate_sudoku(size as u32, 648, 12346).unwrap();
    let init = sol.init_grid;
    for (i, n) in init.iter().enumerate() {
        if i % size == 0 {
            print!("\n");
        }
        print!("{n} ");
    }

    // let solved = sol.solution;
    // for (i, n) in solved.iter().enumerate() {
    //     if i % size == 0 {
    //         print!("\n");
    //     }
    //     print!("{n} ");
    // }
    let a = sol.n_removed;
    println!("\nCells Removed: {a}");
}

// #[test]
// fn random() {
//     Solver::solve(5, 3);
// }
