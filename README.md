# Big Sudoku

<img width="1722" height="986" alt="2026-03-28 23_23_13-Big Sudoku Solver — Mozilla Firefox" src="https://github.com/user-attachments/assets/0a3d4a51-4aae-433e-9586-e746b83cf394" />


[Report PDF](https://docs.google.com/document/d/1vPIDAzfMHhhrVGL4wX_caqKn1JOvpVRGLhHrqRAdBcQ/edit?usp=sharing)

  
## Running the 2 Algorithms Separately
### Dancing Links Generator
```
cd dlx-solver
cargo test --release -- --nocapture
```
In `dlx-solver/src/tests.rs`, you can modify: 
  - grid size
  - % of cells to remove
  - RNG seed

### Brute force Generator
{instructions here}
