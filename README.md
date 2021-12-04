# adventofcode2021
These are my, [Niklas Hallqvist](https://github.com/niklasha) solutions to
[Advent of code 2021](https://adventofcode.com/2021).
They are written in [Rust](https://rust-lang.org).

My reason for doing these are, besides the fact that I like puzzle solving, I want to improve my skills in Rust.

You need Rust, [rustup](https://rustup.rs/) is the suggested way to install Rust, that is about it.
You may need to add some SSL libraries, depending on operating system, but the installation process will tell you, if so.

Run all the days with:
```
cargo run input/
```

Where "input/" is a prefix for the days' inputs, named 01, 02, etc.
The tests (the examples given in the days' descriptions) can be run with:
```
cargo test
```

For every day, the first commit will be the solution with which I solved the puzzle.
After that, I may still revise the code to be more idiomatic or just nicer.


```
My results were:
      --------Part 1--------   --------Part 2--------
Day       Time   Rank  Score       Time   Rank  Score
  4   02:15:50  10583      0   02:51:56  10523      0
  3   00:51:25  13417      0   02:10:52  12509      0
  2   00:14:14   8569      0   00:19:58   8128      0
  1   00:09:46   4878      0   00:14:34   3793      0
```