# Advent of Code 2025 🦀🎄

This repository contains my solutions for the [Advent of Code 2025](https://adventofcode.com/2025), implemented in Rust. It's that wonderful time of year again where we help save Christmas, one puzzle at a time!

## How to Invoke the Project

The project is designed with a CLI that allows you to run specific daily challenges. By default, the program expects input files to be located in the `inputs/` folder, named according to the day (e.g., `inputs/day01.txt`).

- **`<day>`**: The number of the day you want to run (e.g., `1`).
- **`<part>`**: The part of the challenge (`1` or `2`).
- **`[input_path]`** (optional): You can provide a custom path to an input file. If you don't, it will look for `inputs/day{:02}.txt`.

**Example:**
```cargo run -- 1 1```


## Running Integration Tests

Integration tests are located in the `tests/` directory. These are great for verifying that your logic holds up against the example cases provided in the puzzle descriptions.

To run **all** tests across the project:
```cargo test```

To run **one** test:
```cargo test -- test_day01```


## Project Structure

- `src/main.rs`: The entry point for the CLI.
- `src/days/`: Contains the logic for each day's solution.
- `inputs/`: Where the puzzle inputs should be stored.
- `tests/`: Integration tests corresponding to each day's challenge.

Happy Puzzling! 🎅✨