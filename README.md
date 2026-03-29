# Big Sudoku
{screenshot of web page}
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
