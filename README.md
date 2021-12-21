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
 21   00:29:45   3097      0   03:50:58   4292      0
 20   02:39:35   4513      0   02:44:08   4306      0
 18   05:58:11   4394      0   06:14:02   4340      0
 17   01:17:00   4772      0   01:59:48   5084      0
 16   02:38:43   5073      0   03:22:42   4937      0
 15   03:58:38   9950      0   04:39:48   7758      0
 14   00:57:20   7920      0   02:19:31   6446      0
 13   02:04:56   9604      0   02:29:28   9619      0
 12   02:51:19   8765      0   04:46:24  10536      0
 11   02:06:03   8500      0   02:18:46   8626      0
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