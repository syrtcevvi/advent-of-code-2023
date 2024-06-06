# Advent of code 2023. Solved in Rust
![Static Badge](https://img.shields.io/badge/days_solved-9-purple)

My [Advent of code 2023](https://adventofcode.com/2023/about) solutions in the Rust programming language.

This repository holds a separate Rust project for each day. Parts of a day can be found in `part_1.rs` and `part_2.rs` files and have *binary* names `part_1` and `part_2` respectively.

Initially (till the day 4) my input-parsers were hand-written (just split, and another split.. and so on). Eventually I started to use the [nom crate](https://docs.rs/nom/latest/nom/) and become so excited of the simplicity it brings to the proccess of parsing arbitrary text, compared to the splitting approach. So, I decided to rewrite previously solved `part_2` with the `nom` and gived a name `part_2_nom.rs` for those files.

Since *day 4* I started to use `nom` initially

# Run solutions
To run a specific part of some day, navigate to the corresponding project folder and run the following:
```
cargo run --release --bin part_1
```
or
```
cargo run --release --bin part_2
```

# Timings 
*TODO*