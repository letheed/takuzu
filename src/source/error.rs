/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use grid::error::GridParseError;
use std::convert::From;
use std::io::Error as IoError;

/// An error returned by the `source` method when either reading or parsing failed.
#[derive(Debug, Fail)]
pub enum SourceError {
    /// Reading from the source failed.
    #[fail(display = "read failed")]
    Io(#[cause] IoError),
    /// Parsing failed.
    #[fail(display = "parsing failed")]
    Parsing(#[cause] GridParseError),
}

impl From<IoError> for SourceError {
    fn from(err: IoError) -> Self {
        SourceError::Io(err)
    }
}

impl From<GridParseError> for SourceError {
    fn from(err: GridParseError) -> Self {
        SourceError::Parsing(err)
    }
}
