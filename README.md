# AOC 2024

Advent of Code 2024 Solutions. Solutions can be run using:

```bash
cargo run -- --day <DAY> --part <PART>
```

## New day script

Create template for day solution using

```bash
./new_day.sh <DAY>
```

which creates a new puzzle input file, a new `src/day_<DAY>.rs` file for the
solution, and adds a hook into `src/main.rs` for the solution to be run in the
command line.

