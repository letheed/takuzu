/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::convert::From;
use std::error::Error;
use std::fmt::Display;
use std::io::Error as IOError;

use grid::error::GridParseError;

/// An error returned by the `source` method when either reading or parsing failed.
#[derive(Debug)]
pub enum SourceError {
    /// Reading from the source failed.
    IO(IOError),
    /// Parsing failed.
    Parsing(GridParseError),
}

impl Display for SourceError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            SourceError::IO(ref err) => write!(f, "{}: {}", self.description(), err),
            SourceError::Parsing(ref err) => write!(f, "{}: {}", self.description(), err),
        }
    }
}

impl Error for SourceError {
    fn description(&self) -> &str {
        match *self {
            SourceError::IO(_) => "read failed",
            SourceError::Parsing(_) => "parsing failed"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            SourceError::IO(ref err) => Some(err),
            SourceError::Parsing(ref err) => Some(err),
        }
    }
}

impl From<IOError> for SourceError {
    fn from(err: IOError) -> Self {
        SourceError::IO(err)
    }
}

impl From<GridParseError> for SourceError {
    fn from(err: GridParseError) -> Self {
        SourceError::Parsing(err)
    }
}
