use std::{default::Default, ops::Not};

use Cell::{Empty, One, Zero};

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
    #[must_use]
    pub const fn is_empty(self) -> bool {
        matches!(self, Empty)
    }

    /// Returns `true` if a cell is `Zero` or `One`.
    #[must_use]
    pub const fn is_filled(self) -> bool {
        !matches!(self, Empty)
    }
}
