use grid::Grid;
use std::io::{Error, ErrorKind, Read, Result as IOResult};

impl<T> Source for T where T: Read {}

/// The `Source` trait allows to use any implementor of the `Read` trait (a.k.a. 'reader')
/// as an input source for the grid string format with no additional effort.
///
/// `read` parse the input, creates a `Grid` representation and returns it.
///
/// `read_checked` is provided for convenience. On top of `read`, it performs
/// the `check_size` and `check_rules`routines to make sure that the grid
/// is safe to manipulate and legal. This is important, for non-square grids
/// will cause a panic (see [`Grid`](struct.Grid.html)).
pub trait Source: Read {
    /// Reads from a source until EOF, parses the data as a string,
    /// and returns an unchecked grid.
    ///
    /// # Failure
    ///
    /// Returns an I/O error if the read failed
    /// or if a character other than `0`, `1`, `.` or `\n` was found.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let grid = match Source::read(&mut io::stdin()) {
    ///     Ok(grid) => grid,
    ///     Err(e) => { println!("Error: {}", e); return },
    /// };
    /// ```
    fn read(&mut self) -> IOResult<Grid> {
        let buffer = {
            let mut buffer = String::new();
            try!(self.read_to_string(&mut buffer));
            buffer
        };
        let mut parse_error = false;
        let grid =
            Grid::from_parts(
                buffer
                    .lines()
                    .map(|line|
                         line.chars()
                         .map(|c| match c {
                             '0' => Some(false),
                             '1' => Some(true),
                             '.' => None,
                             _ => { parse_error = true; None }
                         }).collect())
                    .collect());
        if parse_error {
            return Err(Error::new(ErrorKind::Other, "found unexpected character(s)"))
        }
        Ok(grid)
    }

    /// Reads from a source until EOF, parses the data as a string,
    /// then checks the grid for size and legality before returning it.
    ///
    /// # Failure
    ///
    /// Returns an error if either one of `read`, `check_size` and `check_rules`
    /// failed or found an error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let grid = match io::stdin().read_checked() {
    ///     Ok(grid) => grid,
    ///     Err(err) => {
    ///         write!(io::stderr(), "Error: {}\n", err).unwrap();
    ///         return
    ///     },
    /// };
    /// ```
    fn read_checked(&mut self) -> Result<Grid, String> {
        let grid = match Source::read(self) {
            Ok(grid) => grid,
            Err(err) => { return Err(format!("{}", err)) }
        };
        if let Some(err) = grid.check_size().err() {
            return Err(format!("{}", err))
        }
        if !grid.check_rules() {
            return Err("this grid is illegal".to_owned())
        }
        Ok(grid)
    }
}
