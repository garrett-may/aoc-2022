# aoc-2022
Advent of Code 2022

Solutions to be written in Rust, using the Cargo package.

To clip/format/run a particular solution:
```
cd days/day_<day_number>/
```

And then on the Stable toolchain:
```
cargo clippy
cargo fmt
cargo run
```

or if using the Nightly toolchain:
```
cargo +nightly clippy
cargo +nightly fmt
cargo +nightly run
```

Note: please head to the correct directory (`days/day_<day_number>/`) when using a solution. This is not important for clipping (optimising) code, or for formatting code. However, it is important for running code, as the filepaths for the resources (the test input and real imput) are located in the source
code.

