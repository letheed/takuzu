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
//! * no more than two of either number adjacent to each other.
//! * each row and each column should contain an equal number of `0`s and `1`s.
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

#![warn(missing_docs)]
#![feature(slice_patterns)]

extern crate libc;

use std::io::{stderr, Write};

pub use grid::{Array, Grid};
pub use source::Source;

mod grid;
mod source;

/// Returns `true` if `stdout` is a terminal.
pub fn isatty_stdout() -> bool {
    match unsafe { libc::isatty(libc::STDOUT_FILENO) } {
        1 => true,
        _ => false,
    }
}

/// Main routine for solving a grid from a source.
///
/// Reads a grid from a source, triggers the solving algorithm and prints
/// the result with colors if appropriate (if `stdout` is a terminal).
///
/// If an error was found, prints it on `stderr` and returns.
pub fn solve_from(source: &mut Source) {
    let grid = match source.source() {
        Ok(grid) => grid,
        Err(err) => {
            write!(stderr(), "Error: {}\n", err.0).unwrap();
            return
        },
    };
    let solutions = grid.solve();
    if solutions.len() == 0 { write!(stderr(), "no solution\n").unwrap(); }
    else if solutions.len() == 1 {
        if isatty_stdout() { print!("{}", solutions[0].to_string_diff(&grid)); }
        else { print!("{}", solutions[0]); }
    }
    else {
        if isatty_stdout() {
            write!(stderr(), "solution 1\n").unwrap();
            print!("{}", solutions[0].to_string_diff(&grid));
            for (i, sol) in solutions.into_iter().enumerate().skip(1) {
                write!(stderr(), "\nsolution {}", i + 1).unwrap();
                print!("\n{}", sol.to_string_diff(&grid));
            }
        }
        else {
            print!("{}", solutions[0]);
            for sol in solutions.into_iter() { print!("\n{}", sol); }
        }
    }
}
