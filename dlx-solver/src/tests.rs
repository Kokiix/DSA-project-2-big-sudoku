use crate::generate_sudoku;

#[test]
fn print_solved() {
    let size = 36;
    let sol = generate_sudoku(size as u32, 0.1, 1).unwrap();
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
