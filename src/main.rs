//! Command-line Chomp player
//! Your Name and Bart Massey 2023
//!
//! # Game loop
//!
//! The human moves first. Each turn:
//!
//! 1. Display the current board.
//! 2. Prompt for a legal human move, retrying invalid input.
//! 3. Make the human move. If poison was eaten, announce that the human
//!    lost (for example, "you lose") and exit successfully immediately.
//! 4. Otherwise, make any winning computer move if one exists. When
//!    none exists, eat the rightmost square in the lowest nonempty row.
//!    If only poison remains, this fallback eats the poison square.
//! 5. Report the computer's move as a zero-based row followed by a
//!    column, identifying it as the computer's move. For example,
//!    "computer: 1 2". If poison was eaten, announce that the human won
//!    (for example, "you win") and exit successfully immediately.
//!
//! Check for game over after each move, before any further move or
//! input prompt. The next turn displays the board after the computer's
//! move. Additional board displays after human moves, on retries, or
//! at game end are allowed, but must always show the current state.
//!
//! # Interaction
//!
//! Prompt and diagnostic wording, capitalization, punctuation, and
//! spacing are flexible. Identify prompts as requests for a move and
//! computer replies as computer moves. Keep each computer reply on its
//! own line, with its row and column in that order. Keep prompts,
//! replies, diagnostics, and results outside the board grid.
//! Write boards, prompts, computer moves, and results to standard output.
//! Flush output before waiting for input so prompts and boards are
//! visible to a human or a PTY-based grader. Use ordinary text output;
//! do not require terminal cursor movement, colors, or screen clearing.
//!
//! End-of-input before the game finishes cancels the game: exit with
//! status zero without announcing a winner or requesting more input.
//! An actual input/output error must produce a diagnostic on standard
//! error and a nonzero exit status, without panicking. A completed game
//! exits with status zero whether the human won or lost.
//!
//! For example, a complete one-row, two-column game might look like
//! this, including the human's input echoed by the terminal:
//!
//! ```text
//! ##
//! Your move (row col): 0 1
//! Computer: 0 0
//! You win!
//! ```

use chomp_ai::*;

/// Display the board as one line per logical row, from top to bottom.
/// Each row has one `#` for each uneaten square and one `.` for each
/// eaten square, from left to right. Whitespace between markers or
/// around rows is allowed. Put labels and other text outside the grid.
/// Print each row on a separate line, including a newline after the last
/// row. For example, this is a four-row, five-column board:
///
/// ```text
/// #####
/// #####
/// ####.
/// #....
/// ```
///
/// # Errors
/// Return an input/output error if writing the board fails.
fn show_posn(posn: &Chomp) -> std::io::Result<()> {
    todo!("display the board {posn:?}")
}

/// Prompt for a legal human move without changing the board.
/// Read one line containing exactly two nonnegative decimal integers:
/// a zero-based row followed by a column, separated by whitespace.
/// Accept leading and trailing whitespace, multiple spaces, and tabs.
/// For example:
///
/// ```text
/// 2 3
/// ```
/// Return `Ok(Some((row, col)))` for a legal move: the selected square
/// must be uneaten and inside the logical board. Selecting `(0, 0)` is
/// legal while the poison remains, although it loses the game.
/// Blank lines, missing/extra fields, negative or unparseable numbers,
/// out-of-range coordinates, and eaten squares are invalid input.
/// Explain invalid input and prompt again without changing the board
/// or allowing a computer turn. Flush output before each input read.
///
/// Return `Ok(None)` on end-of-input, including while retrying a move.
/// Do not treat end-of-input as invalid input and retry forever.
///
/// # Errors
/// Return an input/output error if reading or flushing fails.
fn user_move(posn: &Chomp) -> std::io::Result<Option<(usize, usize)>> {
    todo!("read a move for the board {posn:?}")
}

/// Play a game, as described above.
///
/// Accept exactly two command-line arguments: rows, then columns.
/// Both must be decimal integers accepted by `Chomp::new`: `1..=4`
/// rows and `1..=5` columns. Missing or extra arguments, negative or
/// unparseable values, zero, and oversized dimensions are errors.
/// On an argument error, print a diagnostic to standard error and exit
/// with a nonzero status without starting a game or panicking.
/// Handle any library or input/output errors without panicking.
///
/// Thus, a typical run of the program on a 3×4 board might look like
/// ```text
/// cargo run -- 3 4
/// ```
fn main() {
    // Keep these helpers referenced until the game loop is implemented.
    let _ = (show_posn, user_move);
    todo!()
}
