use crate::{generate_sudoku, solve_sudoku};

#[test]
fn print_solved() {
    // Parameters
    // ========================================================
    let size: usize = 9;
    let remove_proportion = 0.1;
    let seed = 1;
    // ========================================================

    let sol = generate_sudoku(size as u32, remove_proportion, seed);
    let init = sol.init_grid.clone();
    for (i, n) in init.iter().enumerate() {
        if i % size == 0 {
            print!("\n");
        }
        print!("{n} ");
    }

    let sol_attempt = solve_sudoku(size as u32, init);
    println!("Sol solution: {:?}", sol.solution);
    println!("Sol attempt:  {:?}", sol_attempt);
    assert!(sol_attempt != sol.init_grid);
    assert!(sol_attempt == sol.solution);
}
