#![warn(missing_docs)]
#![warn(rust_2018_idioms)]
#![deny(unsafe_code)]

//! A library for solving Takuzu (a.k.a. Binairo) number puzzles.
//!
//! # About
//!
//! Takuzu is a number puzzle, sometimes called binary sudoku.
//! The objective is to fill a grid with `0`s and `1`s while
//! observing the following rules:
//!
//! * no more than two zeros or two ones adjacent to each other in any direction.
//! * each row and each column must contain an equal number of `0`s and `1`s.
//! * no two rows and no two columns are the same.
//!
//! The grids are squares of even size.
//! A valid grid must have one and only one solution.
//! The solver will find and return all valid solutions though.
//!
//! # Input format
//!
//! For parsing, the grids must be represented with the following characters:
//! `0`, `1`, `.` for a missing number, and one `\n` at the end of each row.
//! The final `\n` may be omitted.
//!
//! [Example grids](https://github.com/letheed/takuzu/tree/master/grids)

pub use ansi::ANSIGridDiff;
pub use grid::{
    cell::Cell,
    error::{GridError, GridParseError, GridSizeError},
    Grid,
};

mod ansi;
mod grid;
