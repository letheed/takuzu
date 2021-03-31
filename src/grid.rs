use std::{
    fmt::{self, Display},
    ops::{Index, IndexMut},
    str::FromStr,
};

use cell::Cell;
use error::{GridError, GridParseError, GridSizeError};
use Cell::*;

pub(crate) mod cell;
pub(crate) mod error;

/// An opaque container for manipulating takuzu grids.
///
/// It provides the internal logic and other convenience functions.
/// To create a `Grid` you can:
///
/// * create an empty one yourself with [`new`](#method.new).
/// * use the [`FromStr`](#impl-FromStr) trait, e.g. by calling [`parse`][parse] on a string.
///
/// You can modify the cells as you like.
/// Grids that break the rules will not be solved though.
///
/// [parse]: https://doc.rust-lang.org/stable/std/primitive.str.html#method.parse
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Grid {
    cells: Box<[Cell]>,
    size: usize,
}

impl Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use std::fmt::Write;

        for row in self.cells.chunks(self.size) {
            for cell in row {
                let c = match cell {
                    Zero => '0',
                    One => '1',
                    Empty => '.',
                };
                f.write_char(c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = Cell;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.cells[i * self.size + j]
    }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        &mut self.cells[i * self.size + j]
    }
}

impl FromStr for Grid {
    type Err = GridParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use GridParseError::*;
        use GridSizeError::*;

        if s.is_empty() {
            return Err(BadSize(EmptyGrid));
        }
        let lines: Vec<_> = s.lines().collect();
        let size = lines.len();
        if size % 2 == 1 {
            return Err(BadSize(OddNumberSize(size)));
        }
        let mut cells = Vec::with_capacity(size * size);
        for (i, line) in lines.iter().enumerate() {
            let mut count: usize = 0;
            for c in line.chars() {
                cells.push(match c {
                    '0' => Zero,
                    '1' => One,
                    '.' => Empty,
                    _ => return Err(UnexpectedCharacter(c)),
                });
                count += 1;
            }
            if count != size {
                return Err(BadSize(NotASquare { line: i + 1, found: count, expected: size }));
            }
        }
        Ok(Self::from_parts(cells, size))
    }
}

impl Grid {
    /// Creates an new empty grid of a given size.
    ///
    /// # Errors
    ///
    /// Returns an error if the size is an odd number or 0.
    pub fn new(size: usize) -> Result<Self, GridSizeError> {
        use GridSizeError::*;

        if size == 0 {
            Err(EmptyGrid)
        } else if size % 2 == 1 {
            Err(OddNumberSize(size))
        } else {
            Ok(Self::from_parts(vec![Empty; size * size], size))
        }
    }

    /// Returns the number of rows/columns of the array.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Extracts a slice containing the entire underlying array.
    pub fn as_slice(&self) -> &[Cell] {
        &self.cells
    }

    /// Extracts a mutable slice of the entire underlying array.
    pub fn as_mut_slice(&mut self) -> &mut [Cell] {
        &mut self.cells
    }

    /// Returns `true` if the grid contains no `Empty` cell.
    pub fn is_filled(&self) -> bool {
        !self.cells.contains(&Empty)
    }

    /// Verifies that the grid does not currently violate any of the rules.
    ///
    /// Returns `true` if the grid is legal.
    pub fn is_legal(&self) -> bool {
        self.check_rule1() && self.check_rule2() && self.check_rule3()
    }

    /// Verifies that a certain cell does not violate any of the rules.
    ///
    /// Returns `true` if the value is legal.
    pub fn is_cell_legal(&self, coord: (usize, usize)) -> bool {
        self[coord].is_empty() || {
            self.check_cell_rule1(coord)
                && self.check_cell_rule2(coord)
                && self.check_cell_rule3(coord)
        }
    }

    /// Returns the coordinates of the first `Empty` cell
    /// or `None` if the grid is filled.
    pub fn next_empty(&self) -> Option<(usize, usize)> {
        for (i, cell) in self.cells.iter().enumerate() {
            if cell.is_empty() {
                let row = i / self.size;
                let col = i % self.size;
                return Some((row, col));
            }
        }
        None
    }

    /// Solves the grid using both rules logic and a backtracking algorithm.
    ///
    /// Returns an array containing the solution(s), or an empty array if there
    /// are none.
    ///
    /// # Errors
    ///
    /// Returns an error before any attempt at solving if
    /// the grid breaks any of the rules
    /// (i.e. if [`is_legal`](#method.is_legal) is false).
    pub fn solve(&self) -> Result<Vec<Self>, GridError> {
        if !self.is_legal() {
            return Err(GridError::Illegal);
        }
        let (mut stack, mut solutions) = (Vec::new(), Vec::new());
        let mut grid = self.clone();
        while grid.apply_rules() {}
        stack.push(grid);
        while !stack.is_empty() {
            let mut grid = stack.pop().unwrap();
            match grid.next_empty() {
                Some(coord) => {
                    grid[coord] = One;
                    if grid.is_cell_legal(coord) {
                        let mut grid = grid.clone();
                        while grid.apply_rules() {}
                        stack.push(grid);
                    }
                    grid[coord] = Zero;
                    if grid.is_cell_legal(coord) {
                        while grid.apply_rules() {}
                        stack.push(grid);
                    }
                }
                None => {
                    if grid.is_legal() {
                        solutions.push(grid);
                    }
                }
            }
        }
        Ok(solutions)
    }
}

impl Grid {
    /// Creates a `Grid` from a `Vec` of `Cell`s
    /// and the size of the grid.
    ///
    /// # Panics
    ///
    /// Panics if:
    ///
    /// * size is 0
    /// * size is odd
    /// * the number of cells is not size²
    fn from_parts(cells: Vec<Cell>, size: usize) -> Self {
        assert_ne!(size, 0, "attempted to create an empty grid");
        assert_eq!(size % 2, 0, "attempted to create an odd sized grid");
        assert_eq!(
            cells.len(),
            size * size,
            "putative grid size does not match the number of cells"
        );
        Self { cells: cells.into_boxed_slice(), size }
    }

    /// Verifies that the grid abides by rule 1.
    ///
    /// Rule 1: no more than two of either number adjacent to each other
    /// (both vertically and horizontally).
    fn check_rule1(&self) -> bool {
        for row in self.cells.chunks(self.size) {
            for triplet in row.windows(3) {
                let cell = triplet[0];
                if cell.is_filled() && cell == triplet[1] && cell == triplet[2] {
                    return false;
                }
            }
        }
        for i in 0..self.size - 2 {
            for j in 0..self.size {
                let cell = self[(i, j)];
                if cell.is_filled() && cell == self[(i + 1, j)] && cell == self[(i + 2, j)] {
                    return false;
                }
            }
        }
        true
    }

    /// Verifies that the grid abides by rule 2.
    ///
    /// Rule 2: each row and each column should contain an equal number
    /// of 0s and 1s.
    fn check_rule2(&self) -> bool {
        let nmax = self.size / 2;
        for row in self.cells.chunks(self.size) {
            let count = row.iter().fold((0, 0), |mut count, cell| {
                match cell {
                    Zero => count.0 += 1,
                    One => count.1 += 1,
                    Empty => {}
                }
                count
            });
            if count.0 > nmax || count.1 > nmax {
                return false;
            }
        }
        for i in 0..self.size {
            let mut count = (0, 0);
            for j in 0..self.size {
                match self[(j, i)] {
                    Zero => count.0 += 1,
                    One => count.1 += 1,
                    Empty => {}
                }
            }
            if count.0 > nmax || count.1 > nmax {
                return false;
            }
        }
        true
    }

    /// Verifies that the grid abides by rule 3.
    ///
    /// Rule 3: no two rows and no two columns can be the same.
    fn check_rule3(&self) -> bool {
        for i in 0..self.size - 1 {
            for j in i + 1..self.size {
                if (0..self.size).all(|k| self[(i, k)].is_filled() && self[(i, k)] == self[(j, k)])
                {
                    return false;
                }
                if (0..self.size).all(|k| self[(k, i)].is_filled() && self[(k, i)] == self[(k, j)])
                {
                    return false;
                }
            }
        }
        true
    }

    /// Verifies that the cell with the given coordinates abides by rule 1.
    ///
    /// Rule 1: no more than two of either number adjacent to each other
    /// (both vertically and horizontally).
    fn check_cell_rule1(&self, (row, col): (usize, usize)) -> bool {
        use std::cmp::min;

        for i in row.saturating_sub(2)..min(row + 1, self.size - 2) {
            let cell = self[(i, col)];
            if cell.is_filled() && cell == self[(i + 1, col)] && cell == self[(i + 2, col)] {
                return false;
            }
        }
        for j in col.saturating_sub(2)..min(col + 1, self.size - 2) {
            let cell = self[(row, j)];
            if cell.is_filled() && cell == self[(row, j + 1)] && cell == self[(row, j + 2)] {
                return false;
            }
        }
        true
    }

    /// Verifies that the cell with the given coordinates abides by rule 2.
    ///
    /// Rule 2: each row and each column should contain an equal number
    /// of 0s and 1s.
    fn check_cell_rule2(&self, (row, col): (usize, usize)) -> bool {
        let nmax = self.size / 2;
        let mut count = (0, 0, 0, 0);
        for k in 0..self.size {
            match self[(row, k)] {
                Zero => count.0 += 1,
                One => count.1 += 1,
                Empty => {}
            }
            match self[(k, col)] {
                Zero => count.2 += 1,
                One => count.3 += 1,
                Empty => {}
            }
        }
        count.0 <= nmax && count.1 <= nmax && count.2 <= nmax && count.3 <= nmax
    }

    /// Verifies that the cell with the given coordinates abides by rule 3.
    ///
    /// Rule 3: no two rows and no two columns can be the same.
    fn check_cell_rule3(&self, (row, col): (usize, usize)) -> bool {
        let rows_abide =
            (0..self.size).filter(|&i| i != row && self[(i, col)] == self[(row, col)]).all(|i| !{
                (0..self.size).all(|j| self[(row, j)].is_filled() && self[(row, j)] == self[(i, j)])
            });
        let cols_abide =
            (0..self.size).filter(|&j| j != col && self[(row, j)] == self[(row, col)]).all(|j| !{
                (0..self.size).all(|i| self[(i, col)].is_filled() && self[(i, col)] == self[(i, j)])
            });
        rows_abide && cols_abide
    }
}

impl Grid {
    /// Skims through the grid once, filling in the blanks
    /// where the value is unambiguous according to one of the rules,
    /// then returns if the grid was modified or repeats the operation
    /// for the next rule. Each rule is applied once at the most.
    ///
    /// Returns `true` if the grid was modified.
    ///
    /// # Warning
    ///
    /// Does not guarantee the legality of the modifications.
    /// For performance reasons, deductions made from a rule are not
    /// checked for legality against the other rules. This can result in
    /// grids with no legal solution being filled illegally.
    /// Grids with one or more legal solution(s) are not affected.
    fn apply_rules(&mut self) -> bool {
        self.apply_rule1() || self.apply_rule2() || self.apply_rule3()
    }

    /// Disambiguates empty cells after rule 1.
    ///
    /// Rule 1: no more than two of either number adjacent to each other
    /// (both vertically and horizontally).
    #[rustfmt::skip]
    fn apply_rule1(&mut self) -> bool {
        let mut rule_applied = false;
        for i in 0..self.size {
            for j in 0..self.size - 2 {
                let trio = (self[(i, j)], self[(i, j + 1)], self[(i, j + 2)]);
                match trio {
                    (Empty, Zero, Zero) => { self[(i, j  )] = One;  rule_applied = true; }
                    (Zero, Empty, Zero) => { self[(i, j+1)] = One;  rule_applied = true; }
                    (Zero, Zero, Empty) => { self[(i, j+2)] = One;  rule_applied = true; }
                    (Empty, One, One)   => { self[(i, j  )] = Zero; rule_applied = true; }
                    (One, Empty, One)   => { self[(i, j+1)] = Zero; rule_applied = true; }
                    (One, One, Empty)   => { self[(i, j+2)] = Zero; rule_applied = true; }
                    _ => {},
                }
                let trio = (self[(j, i)], self[(j + 1, i)], self[(j + 2, i)]);
                match trio {
                    (Empty, Zero, Zero) => { self[(j  , i)] = One;  rule_applied = true; }
                    (Zero, Empty, Zero) => { self[(j+1, i)] = One;  rule_applied = true; }
                    (Zero, Zero, Empty) => { self[(j+2, i)] = One;  rule_applied = true; }
                    (Empty, One, One)   => { self[(j  , i)] = Zero; rule_applied = true; }
                    (One, Empty, One)   => { self[(j+1, i)] = Zero; rule_applied = true; }
                    (One, One, Empty)   => { self[(j+2, i)] = Zero; rule_applied = true; }
                    _ => {},
                }
            }
        }
        rule_applied
    }

    /// Disambiguates empty cells after rule 2.
    ///
    /// Rule 2: each row and each column should contain an equal number
    /// of 0s and 1s.
    fn apply_rule2(&mut self) -> bool {
        let mut rule_applied = false;
        let nmax = self.size / 2;
        for i in 0..self.size {
            let mut count = (0, 0, 0, 0);
            for j in 0..self.size {
                match self[(i, j)] {
                    Zero => count.0 += 1,
                    One => count.1 += 1,
                    Empty => {}
                }
                match self[(j, i)] {
                    Zero => count.2 += 1,
                    One => count.3 += 1,
                    Empty => {}
                }
            }
            if count.0 == nmax && count.1 != nmax {
                rule_applied = true;
                for j in 0..self.size {
                    if self[(i, j)].is_empty() {
                        self[(i, j)] = One;
                    }
                }
            } else if count.1 == nmax && count.0 != nmax {
                rule_applied = true;
                for j in 0..self.size {
                    if self[(i, j)].is_empty() {
                        self[(i, j)] = Zero;
                    }
                }
            }
            if count.2 == nmax && count.3 != nmax {
                rule_applied = true;
                for j in 0..self.size {
                    if self[(j, i)].is_empty() {
                        self[(j, i)] = One;
                    }
                }
            } else if count.3 == nmax && count.2 != nmax {
                rule_applied = true;
                for j in 0..self.size {
                    if self[(j, i)].is_empty() {
                        self[(j, i)] = Zero;
                    }
                }
            }
        }
        rule_applied
    }

    /// Disambiguates empty cells after rule 3.
    ///
    /// Rule 3: no two rows and no two columns can be the same.
    fn apply_rule3(&mut self) -> bool {
        macro_rules! row {
            ($i:expr) => {
                self.cells[$i * self.size..($i + 1) * self.size]
            };
        }

        let size = self.size;
        let mut rule_applied = false;
        for i in 0..size {
            if row!(i).iter().filter(|value| value.is_empty()).count() == 2 {
                for l in 0..size {
                    if l != i
                        && !row!(l).contains(&Empty)
                        && row!(i)
                            .iter()
                            .zip(row!(l).iter())
                            .filter(|&(value, _)| value.is_filled())
                            .all(|(value, other)| value == other)
                    {
                        for j in 0..size {
                            if self[(i, j)].is_empty() {
                                self[(i, j)] = !self[(l, j)];
                            }
                        }
                        rule_applied = true;
                        break;
                    }
                }
            }
            let j = i;
            if (0..size).filter(|&l| self[(l, j)].is_empty()).count() == 2 {
                for m in 0..size {
                    if m != j
                        && (0..size).all(|i| self[(i, m)].is_filled())
                        && (0..size)
                            .filter(|&i| self[(i, j)].is_filled())
                            .all(|i| self[(i, j)] == self[(i, m)])
                    {
                        for i in 0..size {
                            if self[(i, j)].is_empty() {
                                self[(i, j)] = !self[(i, m)];
                            }
                        }
                        rule_applied = true;
                        break;
                    }
                }
            }
        }
        rule_applied
    }
}
