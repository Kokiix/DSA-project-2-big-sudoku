use crate::generate_sudoku;

#[test]
fn print_solved() {
    // Parameters
    // ========================================================
    let size = 36;
    let remove_proportion = 0.1;
    let seed = 1;
    // ========================================================

    let sol = generate_sudoku(size as u32, remove_proportion, seed).unwrap();
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
