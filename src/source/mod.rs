/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use grid::Grid;
use self::error::SourceError;
use std::io::Read;

pub mod error;

impl<T: ?Sized + Read> Source for T {}

/// The `Source` trait allows to use any implementor of the `Read` trait
/// as an input source for the grid string format with no additional effort.
pub trait Source: Read {
    /// Creates a `Grid` from a readable source.
    ///
    /// Reads from the source until EOF, parses the data as a string,
    /// checking that the size is correct, then decodes it and returns `Grid`.
    ///
    /// # Errors
    ///
    /// Returns an error if either the read failed,
    /// a character other than `0`, `1`, `.` or `\n` was found,
    /// or the array isn't properly sized.
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
        let mut buffer = String::new();
        self.read_to_string(&mut buffer)?;
        buffer.parse().map_err(Into::into)
    }
}
