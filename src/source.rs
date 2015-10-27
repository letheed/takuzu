use std::io::Read;

use grid::{Array, Grid};

impl<T> Source for T where T: Read {}

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
    /// If the read and the parsing were successful, the faulty array
    /// is returned as well.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let grid = match io::stdin().source() {
    ///     Ok(grid) => grid,
    ///     Err(e) => {
    ///         write!(io::stderr(), "Error: {}\n", e.0).unwrap();
    ///         return
    ///     },
    /// };
    /// ```
    fn source(&mut self) -> Result<Grid, (String, Option<Array>)> {
        let buffer = {
            let mut buffer = String::new();
            match self.read_to_string(&mut buffer) {
                Err(err) => { return Err((format!("{}", err), None)) }
                _ => {}
            }
            buffer
        };
        let mut parse_error = false;
        let array = buffer.lines().map(|line| line.chars()
                                       .map(|c| match c {
                                           '0' => Some(false),
                                           '1' => Some(true),
                                           '.' => None,
                                           _ => { parse_error = true; None }
                                       }).collect())
                                  .collect();
        if parse_error {
            return Err(("found unexpected character(s)".to_owned(), None))
        }
        Grid::new(array).map_err(|err| (err.0, Some(err.1)))
    }
}
