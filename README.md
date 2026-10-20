# Chomp Skeleton
Your Name and Bart Massey 2023

This is starter code for the game of
[Chomp](https://en.wikipedia.org/wiki/Chomp). Complete the
library and the command-line program to play against its AI.

The Rustdoc comments in `src/lib.rs` define the library interface
and behavior for automated testing. Keep those public types and
method signatures. You may choose the board representation and
add helper functions or methods.

The comments in `src/main.rs` outline a simple playable CLI.
Choose your own prompts, input format, and display, and adapt its
private helpers as needed. Submit a complete gameplay transcript
as `gameplay.txt` for human review of the CLI.

See [sample-gameplay.txt](sample-gameplay.txt) for a complete
three-row, two-column game recorded from the reference solution.
Your prompts and board display may look different.

Build with `cargo build`. If you use command-line size arguments,
run a three-row, four-column game with `cargo run -- 3 4`.
The skeleton compiles, but its `todo!()` placeholders and game
tests require your implementation.

*Update this README to describe your implementation, testing,
and any issues you encountered. Then **delete this comment**.*

# License

This work is licensed under the "MIT License". Please see
`LICENSE.txt` in this distribution for license terms.
