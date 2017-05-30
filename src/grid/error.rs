/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use self::GridError::*;
use self::GridParseError::*;
use self::GridSizeError::*;
use std::convert::From;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// An error returned when parsing a string to create a grid failed.
#[derive(Debug)]
pub enum GridParseError {
    /// The grid does not have the right size.
    ///
    /// (It should be square, of non-null, even size.)
    BadSize(Box<GridSizeError>),
    /// At least one character other than `0`, `1`, `.` or `\n`
    /// was found in the string.
    UnexpectedCharacter,
}

impl Display for GridParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            BadSize(ref err)    => write!(f, "{}: {}", self.description(), err),
            UnexpectedCharacter => f.write_str(self.description()),
        }
    }
}

impl Error for GridParseError {
    fn description(&self) -> &str {
        match *self {
            BadSize(_)          => "faulty grid size",
            UnexpectedCharacter => "found unexpected character(s)",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            BadSize(ref err)    => Some(err),
            UnexpectedCharacter => None,
        }
    }
}

impl From<GridSizeError> for GridParseError {
    fn from(err: GridSizeError) -> Self {
        BadSize(Box::new(err))
    }
}

/// An error returned when checking if the grid is legal.
#[derive(Debug)]
pub enum GridError {
    /// The grid is illegal, that is it infringes at least one of the rules.
    Illegal,
}

impl Display for GridError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            Illegal => f.write_str(self.description()),
        }
    }
}

impl Error for GridError {
    fn description(&self) -> &str {
        match *self {
            Illegal => "grid is illegal",
        }
    }
}

/// An error returned when the grid is not properly sized.
#[derive(Debug)]
pub enum GridSizeError {
    /// The grid is empty.
    EmptyGrid,
    /// The grid is not a square.
    /// The field contains the line number that triggered the error.
    NotASquare(usize),
    /// The size of the grid is an odd number.
    OddNumberSize,
}

impl Display for GridSizeError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            EmptyGrid | OddNumberSize => f.write_str(self.description()),
            NotASquare(n) => write!(f, "{} (line {})", self.description(), n),
        }
    }
}

impl Error for GridSizeError {
    fn description(&self) -> &str {
        match *self {
            EmptyGrid     => "grid is empty",
            NotASquare(_) => "grid is not a square",
            OddNumberSize => "grid size is an odd number",
        }
    }
}
