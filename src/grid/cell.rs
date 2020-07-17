use std::{default::Default, ops::Not};
use Cell::*;

///  An enum representing the state of a cell.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Cell {
    Zero,
    One,
    Empty,
}

impl Default for Cell {
    fn default() -> Self {
        Empty
    }
}

impl Not for Cell {
    type Output = Self;

    fn not(self) -> Self {
        match self {
            Zero => One,
            One => Zero,
            Empty => Empty,
        }
    }
}

impl Cell {
    /// Returns `true` if a cell is `Empty`.
    pub fn is_empty(self) -> bool {
        match self {
            Empty => true,
            _ => false,
        }
    }

    /// Returns `true` if a cell is `Zero` or `One`.
    pub fn is_filled(self) -> bool {
        match self {
            Empty => false,
            _ => true,
        }
    }
}
