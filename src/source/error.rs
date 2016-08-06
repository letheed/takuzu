/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use self::SourceError::*;
use grid::error::GridParseError;
use std::convert::From;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::Error as IoError;

/// An error returned by the `source` method when either reading or parsing failed.
#[derive(Debug)]
pub enum SourceError {
    /// Reading from the source failed.
    Io(Box<IoError>),
    /// Parsing failed.
    Parsing(Box<GridParseError>),
}

impl Display for SourceError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            Io(ref err)      => write!(f, "{}: {}", self.description(), err),
            Parsing(ref err) => write!(f, "{}: {}", self.description(), err),
        }
    }
}

impl Error for SourceError {
    fn description(&self) -> &str {
        match *self {
            Io(_)      => "read failed",
            Parsing(_) => "parsing failed"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            Io(ref err)      => Some(err),
            Parsing(ref err) => Some(err),
        }
    }
}

impl From<IoError> for SourceError {
    fn from(err: IoError) -> Self {
        Io(Box::new(err))
    }
}

impl From<GridParseError> for SourceError {
    fn from(err: GridParseError) -> Self {
        Parsing(Box::new(err))
    }
}
