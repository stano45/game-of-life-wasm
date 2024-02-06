# Game of Rust

Rust implementation of [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life). This implementation provides three different approaches for calculating the next generation of cells: Naive, HashSet-based, and Parallel. The Hash-Set implementation is not super efficient though.

## Getting Started

To run the Game of Life, follow these steps:

1. Clone the repository to your local machine:

   ```bash
   git clone https://github.com/stano45/game-of-rust.git
   ```

2. Navigate to the project directory:

   ```bash
   cd game-of-rust
   ```

3. Build the project using `cargo`:

   ```bash
   cargo build --release
   ```

4. Run the game with the desired parameters. You can specify the width, height, number of iterations, and implementation type:

   ```bash
   cargo run --release -- <width> <height> <iterations> <implementation> [seed_file]
   ```

   Replace `<width>`, `<height>`, `<iterations>`, and `<implementation>` with your preferred values. The `seed_file` argument is optional, and if not provided, the initial state will be generated randomly.

5. After running the game, it will display the progress and write the final state to a text file.

## Implementation Options

This implementation provides three different approaches for calculating the next generation of cells:

- **Naive**: Uses a straightforward nested loop approach to update the cell grid.
- **HashSet-based**: Utilizes a HashSet to optimize neighbor calculations and updates.
- **Parallel**: Parallelizes the computation using the Rayon library for improved performance.

## Example Usage

Here's an example of how to run the game:

```bash
cargo run --release -- 50 30 100 hash seed.txt
```

In this example, the game will simulate a 50x30 grid for 100 iterations using the HashSet-based implementation. If a seed file (`seed.txt`) is provided, it will use that as the initial state; otherwise, it will generate a random initial state.

The seed file should contain a grid of cells, where `.` represents a dead cell and `O` represents a live cell. The grid should be the same size as the specified width and height.

## Saving the Final State

The final state of the simulation will be saved to a text file in the current directory. The filename will include the width, height, and total number of iterations, making it easy to identify different simulations.

The first line of the file will contain the width and height, and the iteration index, and the subsequent lines will contain the final state of the grid.

## Contributing

If you'd like to contribute to this project, feel free to fork the repository, make your changes, and submit a pull request. Bug reports, suggestions, and improvements are welcome.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.