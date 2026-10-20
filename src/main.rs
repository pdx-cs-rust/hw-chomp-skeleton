//! Command-line Chomp player
//! Your Name and Bart Massey 2023
//!
//! Build a simple interface for playing against your library's AI:
//!
//! 1. Get a board size, using prompts or command-line arguments.
//! 2. Display the board and ask the human for a move.
//! 3. Make the human move. If poison was eaten, announce the loss.
//! 4. Otherwise, get a winning computer move from the library. If none
//!    exists, choose another uneaten nonpoison square if possible, or
//!    eat poison if it is the only square left. Choosing the rightmost
//!    square in the lowest nonempty row is a simple fallback.
//! 5. Show the computer's move. If poison was eaten, announce the win;
//!    otherwise, continue playing.
//!
//! The human moves first. Stop as soon as either player eats poison.
//! Choose prompts and a board display that make the game easy to play.
//! You may assume the human follows your input instructions; elaborate
//! error handling is not required. Adapt these private helpers as needed.
//!
//! The CLI will be reviewed through your submitted gameplay transcript.
//! Show a complete game with human and computer moves, changing boards,
//! and the final result. The library will be tested automatically.

use chomp_ai::*;

/// Display a readable board. For example, use `#` for uneaten squares
/// and `.` for eaten squares:
///
/// ```text
/// #####
/// #####
/// ####.
/// #....
/// ```
///
/// You may add row and column labels or choose another clear display.
fn show_posn(posn: &Chomp) {
    todo!("display the board {posn:?}")
}

/// Prompt for and read a human move, without changing the board.
/// One simple input format is a zero-based row followed by a column:
///
/// ```text
/// 2 3
/// ```
///
/// Return `Some((row, col))` for a move you can use, or `None` if no
/// move was obtained. The caller can ask again or end the game.
fn user_move(posn: &Chomp) -> Option<(usize, usize)> {
    todo!("read a move for the board {posn:?}")
}

/// Play a game using the library and the helpers above.
/// You may prompt for the board size or accept rows and columns as
/// command-line arguments. For an interface using arguments, a
/// three-row, four-column game could be started with:
///
/// ```text
/// cargo run -- 3 4
/// ```
fn main() {
    // Keep these helpers referenced until the game loop is implemented.
    let _ = (show_posn, user_move);
    todo!()
}
