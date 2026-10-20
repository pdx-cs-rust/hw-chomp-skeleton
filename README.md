# Chomp Skeleton
Your Name and Bart Massey 2023

This is starter code for the game of
[Chomp](https://en.wikipedia.org/wiki/Chomp). Complete the
library, then write a command-line program to play against its AI.

The Rustdoc comments in `src/lib.rs` define the library interface
and behavior for automated testing. Keep those public types and
method signatures. You may choose the board representation and
add helper functions or methods.

Follow the [assignment] for CLI instructions. Create `src/main.rs`
for your program; Cargo will detect it as the `chomp` binary.
Choose your own prompts, input format, display, and helper
functions. Submit a complete gameplay transcript as
`gameplay.txt` for human review of the CLI.

[assignment]: https://github.com/pdx-cs-rust/rust-course-notes/blob/main/assignments/chomp/assignment.md

See [sample-gameplay.txt](sample-gameplay.txt) for a complete
three-row, two-column game recorded from the reference solution.
Your CLI should produce a roughly similar trace: an initial
board, human prompts and moves, identified computer moves,
updated boards after each move, and the final result. Your
wording and display layout may differ.

Build the library with `cargo build`. Once you have created the
program, run it with `cargo run`. If you use command-line size
arguments, run a three-row, four-column game with
`cargo run -- 3 4`. The library skeleton compiles, but its
`todo!()` placeholders and game tests require your implementation.

*Update this README to describe your implementation, testing,
and any issues you encountered. Then **delete this comment**.*

# License

This work is licensed under the "MIT License". Please see
`LICENSE.txt` in this distribution for license terms.
