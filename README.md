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
 10   00:51:56   8598      0   01:11:15   8197      0
  9   01:10:53  10638      0   03:08:38  11473      0
  8   00:35:01   8811      0   02:30:49   7528      0
  7   00:14:14   6295      0   00:22:56   5853      0
  6   00:23:42   7371      0   01:27:14   8818      0
  5   02:39:03  12286      0   03:31:45  12389      0
  4   02:15:50  10583      0   02:51:56  10523      0
  3   00:51:25  13417      0   02:10:52  12509      0
  2   00:14:14   8569      0   00:19:58   8128      0
  1   00:09:46   4878      0   00:14:34   3793      0
```