macro_rules! ansi_esc {
    () => {
        '\u{1b}'
    };
}

macro_rules! ansi_color {
    ($color_number:expr) => {
        concat!(ansi_esc!(), '[', $color_number, 'm')
    };
}

macro_rules! ansi_color_reset {
    () => {
        ansi_color!(0)
    };
}

macro_rules! mk_color {
    ($color_number:expr, $str:expr) => {
        concat!(ansi_color!($color_number), $str, ansi_color_reset!())
    };
}

macro_rules! red {
    ($str:expr) => {
        mk_color!(31, $str)
    };
}

macro_rules! yellow {
    ($str:expr) => {
        mk_color!(33, $str)
    };
}
macro_rules! cyan {
    ($str:expr) => {
        mk_color!(36, $str)
    };
}

use crate::{Cell, Grid};
use std::fmt::{self, Display};

/// Displays a colored diff in ANSI terminals.
///
/// The first grid is used as a reference and the second grid will be displayed.
/// Cells in the second grid that differ from the reference will be displayed
/// in color.
///
/// If the grids have different sizes, the second grid will be displayed normally.
///
/// # Warning
///
/// A red-colored cell signals that a `0` or a `1` from the reference grid
/// was overwritten. If `reference` is the original grid and `self`
/// is a solution, this should *never* happen.
#[derive(Copy, Clone, Debug)]
pub struct ANSIGridDiff<'a>(pub &'a Grid, pub &'a Grid);

impl Display for ANSIGridDiff<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ref_size = self.0.size();
        if ref_size != self.1.size() {
            return write!(f, "{}", self.1);
        }
        let ref_rows = self.0.as_slice().chunks(ref_size);
        let rows = self.1.as_slice().chunks(ref_size);
        for (ref_row, row) in ref_rows.zip(rows) {
            for (ref_cell, cell) in ref_row.iter().zip(row) {
                #[rustfmt::skip]
                let s = match cell {
                    Cell::Zero => {
                        // No color if nothing changed.
                        if ref_cell == cell { "0" }
                        // Color for 0 if we filled in a blank.
                        else if ref_cell.is_empty() { cyan!('0') }
                        // Red for error if we overwrote.
                        else { red!('0') }
                    },
                    Cell::One => {
                        // No color if nothing changed.
                        if ref_cell == cell { "1" }
                        // Color for 1 if we filled in a blank.
                        else if ref_cell.is_empty() { yellow!('1') }
                        // Red for error if we overwrote.
                        else { red!('1') }
                    },
                    Cell::Empty => {
                        // No color if nothing changed.
                        if ref_cell == cell { "." }
                        // Red for error if we overwrote.
                        else { red!('.') }
                    }
                };
                f.write_str(s)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
