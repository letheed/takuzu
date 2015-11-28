/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::error::Error;
use std::fmt::Display;
use grid::error::GridParseError;

/// An error returned by the `source` method when either reading or parsing failed.
#[derive(Debug)]
pub enum SourceError {
    /// Reading from the source failed.
    IOError(::std::io::Error),
    /// Parsing failed.
    ParseError(GridParseError),
}

impl Display for SourceError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str(self.description())
    }
}

impl Error for SourceError {
    fn description(&self) -> &str {
        match *self {
            SourceError::IOError(_) => "read failed",
            SourceError::ParseError(_) => "parsing failed"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            SourceError::IOError(ref err) => Some(err),
            SourceError::ParseError(ref err) => Some(err),
        }
    }
}
