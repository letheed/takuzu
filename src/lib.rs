//! A Takuzu (a.k.a. Binairo) solving library.
//!
//! See [tackle](../tackle/index.html) for the solver.
//!
//! # About
//!
//! Takuzu is a number puzzle.
//! The objective is to fill a grid with `0`s and `1`s while
//! observing the following rules:
//!
//! * each row and each column should contain an equal number of `0`s and `1`s.
//! * no more than two of either number adjacent to each other (vertically and horizontally).
//! * no two rows and no two columns can be the same.
//!
//! The grids are squares of even length.
//! A valid grid must have one and only one solution.
//!
//! # Format
//!
//! The grid should be represented with the following characters:
//! `0`, `1`, `.` for a missing number and one `\n` at the end of each row.
//! (The final `\n` can be omitted though.)
//!
//! [Example grids](https://github.com/Letheed/takuzu/tree/master/grids)

#![feature(slice_patterns)]
#![warn(missing_docs)]

extern crate libc;

pub use grid::Grid;
pub use source::Source;

mod grid;
mod source;

use std::io::{stderr, Write};

/// Returns `true` if `stdout` refers to a terminal.
///
/// # Panics
///
/// Panics if `isatty()` returns something other than `0` or `1`.
pub fn isatty_stdout() -> bool {
    match unsafe { libc::isatty(libc::STDOUT_FILENO) } {
        0 => false,
        1 => true,
        _ => { panic!("invalid return value: isatty()"); }
    }
}

/// Main routine for solving a grid from a source.
///
/// Reads a grid from a source, performs standard checks,
/// triggers the solving algorithm and pretty prints the result.
///
/// If an error was found, prints it on `stderr`.
pub fn solve_from(source: &mut Source) {
    let grid_ref: Grid = match source.read_checked() {
        Ok(grid) => grid,
        Err(err) => {
            write!(stderr(), "Error: {}\n", err).unwrap();
            return
        },
    };
    let mut grid = grid_ref.clone();
    grid.solve();
    grid.print_pretty(&grid_ref);
}
