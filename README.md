# Advent of Code Solutions

## Overview
This repository contains my solutions for the Advent of Code (AoC) challenges, implemented in Rust.

## Folder Structure
The project is organized into several directories:

- `src/`: Contains the source code.
  - `examples/`: Sample inputs for quick tests and examples.
  - `inputs/`: Actual input data for each day's challenge.
  - `solutions/`: Rust modules with solutions for each day. Add new day's solution as `dayXX.rs`.
  - `lib.rs`: Library root file.
  - `main.rs`: Main executable for running solutions.

To add a solution for a new day, place the challenge input as `dayXX.txt` in the `inputs` directory and the solution code in `solutions/dayXX.rs`.

## Running
### Prerequisites
- Rust programming environment.
- Cargo, Rust's package manager.

### Running a Solution
1. Clone the repository to your local machine.
2. Navigate to the project directory.
3. To run a solution for a specific day, execute:
   ```
   cargo run -- [day_number]
   ```
   Replace `[day_number]` with the day number of the challenge (e.g., `1` for Day 1).

### Testing and Benchmarking
- **Testing:** Each day's solution includes tests. Run them using:
  ```
  cargo test
  ```
- **Benchmarking:** Performance benchmarks are available for each solution. Run them using:
  ```
  cargo bench
  ```


## License
Distributed under the GNU GPLv3 License. See `LICENSE` for more information.