/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use self::Cell::*;
use std::default::Default;
use std::ops::Not;

/// An enum representing the state of a cell.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Cell {
    /// Stands for `0`.
    Zero,
    /// Stands for `1`.
    One,
    /// Empty cell.
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
            Zero  => One,
            One   => Zero,
            Empty => Empty,
        }
    }
}

impl Cell {
    /// Returns `true` if a cell is `Empty`.
    pub fn is_empty(self) -> bool {
        match self {
            Empty => true,
            _     => false,
        }
    }

    /// Returns `true` if a cell is `Zero` or `One`.
    pub fn is_filled(self) -> bool {
        match self {
            Empty => false,
            _     => true,
        }
    }
}
