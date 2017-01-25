/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use grid::Grid;
use self::error::SourceError;
use std::io::Read;

pub mod error;

/// The `Source` trait allows to use any implementor as an input source.
///
/// # `Read` source
///
/// The `Source` trait allows to use any implementor of the `Read` trait
/// with no additional effort.
/// It reads from the source until EOF, parses the data as a string,
/// checking that the size is correct, then decodes it and returns `Grid`.
///
/// ## Errors
///
/// Returns an error if either the read failed,
/// a character other than `0`, `1`, `.` or `\n` was found,
/// or the array isn't properly sized.
///
/// ## Examples
///
/// ```rust
/// # use std::error::Error;
/// # use std::io;
/// # use std::io::Write;
/// # use takuzu::Source;
/// let grid = match io::stdin().source() {
///     Ok(grid) => grid,
///     Err(err) => {
///         writeln!(io::stderr(), "error: {}", err).unwrap();
///         return
///     },
/// };
/// ```
pub trait Source {
    /// Creates a `Grid` from a source.
    fn source(&mut self) -> Result<Grid, SourceError>;
}

impl<T: ?Sized> Source for T
    where T: Read {
    fn source(&mut self) -> Result<Grid, SourceError> {
        let mut buffer = String::new();
        self.read_to_string(&mut buffer)?;
        buffer.parse().map_err(Into::into)
    }
}
