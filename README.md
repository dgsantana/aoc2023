# Advent of Code 2023 Solutions

Welcome to my repository for solving Advent of Code 2023 challenges! In this repository, you'll find my solutions for each day's problem, along with instructions on how to run them.
Only pure Rust and itertools crate are used on the solutions, I do have some extra crates but are mostly for possible visualizations or future work on visualizations.

## Table of Contents
- [Folder Structure](#folder-structure)
- [How to Use](#how-to-use)
- [Contributing](#contributing)
- [License](#license)

## Folder Structure

The repository follows a specific folder structure for organizing solutions and input data:

/src/solutions/day1.rs
/src/solutions/day2.rs
...
/inputs
day1.txt
day2.txt
...

Each day has its own solution code file (e.g., `/src/solutions/day1.rs`, `/src/solutions/day2.rs`) and the input data file (e.g., `inputs/day1.txt`, `inputs/day2.txt`).
The inputs folder is not included due to AoC copyright, so please provide your own from [AoC2023](https://adventofcode.com/2023)

## How to Use

To run a specific day's solution, for example for day 2 part 1, `cargo run - 2 1`.

It's also possible to benchmark the solutions, just run `cargo bench` for all the days, or `cargo bench dayN` where **N** is the day to benchmark.
Criterion will generate an HTML with the results at target/criterion/report/index.html.

## Contributing
If you'd like to contribute or suggest improvements, feel free to submit a pull request. Contributions are always welcome!

## License
This project is licensed under the MIT License.