/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::io::Read;

use grid::Grid;
use self::error::SourceError;

pub mod error;

impl<T: ?Sized + Read> Source for T {}

/// The `Source` trait allows to use any implementor of the `Read` trait
/// as an input source for the grid string format with no additional effort.
pub trait Source: Read {
    /// Creates a `Grid` from a readable source.
    /// Reads from the source until EOF, parses the data as a string,
    /// then checks the array for size and legality and converts it to a `Grid`
    ///
    /// # Failure
    ///
    /// Returns an error if either the read failed,
    /// a character other than `0`, `1`, `.` or `\n` was found,
    /// or the if the array is invalid (empty or non-square) or illegal.
    /// If the read was successful and no unexpected character was found,
    /// the faulty array is returned as well.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use std::io;
    /// # use std::io::Write;
    /// # use takuzu::Source;
    /// let grid = match io::stdin().source() {
    ///     Ok(grid) => grid,
    ///     Err(err) => {
    ///         write!(io::stderr(), "error: {}\n", err.description()).unwrap();
    ///         return
    ///     },
    /// };
    /// ```
    fn source(&mut self) -> Result<Grid, SourceError> {
        let buffer = {
            let mut buffer = String::new();
            try!(self.read_to_string(&mut buffer));
            buffer
        };
        buffer.parse().map_err(|err| SourceError::Parsing(err))
    }
}
