use crate::{generate_sudoku, solve_sudoku};

#[test]
fn print_solved() {
    // Parameters
    // ========================================================
    let size: usize = 36;
    let remove_proportion = 0.1;
    let seed = 1;
    // ========================================================

    let sol = solve_sudoku(generate_sudoku(size as u32, seed), remove_proportion);
    let init = sol.init_grid;
    for (i, n) in init.iter().enumerate() {
        if i % size == 0 {
            print!("\n");
        }
        print!("{n} ");
    }

    let a = sol.n_removed;
    println!("\nCells Removed: {a}");
}
