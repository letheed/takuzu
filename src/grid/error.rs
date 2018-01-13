/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use std::convert::From;

/// An error returned when parsing a string to create a grid failed.
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash, Fail)]
pub enum GridParseError {
    /// The grid does not have the right size.
    ///
    /// (It should be square, of non-null, even size.)
    #[fail(display = "faulty grid size")]
    BadSize(#[cause] GridSizeError),
    /// At least one character other than `0`, `1`, `.` or `\n`
    /// was found in the string.
    #[fail(display = "found unexpected character(s)")]
    UnexpectedCharacter,
}

impl From<GridSizeError> for GridParseError {
    fn from(err: GridSizeError) -> Self {
        GridParseError::BadSize(err)
    }
}

/// An error returned when checking if the grid is legal.
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash, Fail)]
pub enum GridError {
    /// The grid is illegal, that is it infringes at least one of the rules.
    #[fail(display = "grid is illegal")]
    Illegal,
}

/// An error returned when the grid is not properly sized.
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash, Fail)]
pub enum GridSizeError {
    /// The grid is empty.
    #[fail(display = "grid is empty")]
    EmptyGrid,
    /// The grid is not a square.
    /// The field contains the line number that triggered the error.
    #[fail(display = "grid is not a square (line {})", _0)]
    NotASquare(usize),
    /// The size of the grid is an odd number.
    #[fail(display = "grid size is an odd number")]
    OddNumberSize,
}
