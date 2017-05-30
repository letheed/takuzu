/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use self::cell::Cell;
use self::cell::Cell::*;
use self::error::{GridError, GridParseError, GridSizeError};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::{Index, IndexMut};
use std::str::FromStr;

pub mod cell;
pub mod error;

/// An opaque container for takuzu grid manipulation.
///
/// It provides the internal logic and other convenience functions.
/// To create a `Grid` you can:
///
/// * create an empty one yourself with [`Grid::new(size)`](#method.new).
/// * use the `FromStr` trait, e.g. by calling `.parse()` on a string.
/// * use the [`Source`](trait.Source.html) trait, i.e. by calling `.source()` on any `Read` implementor.
///
/// The `Grid` type does not maintain any internal invariant. That is,
/// you can modify the grid as you like and break the rules.
/// Such grids will not be solved, though.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Grid {
    cells: Vec<Cell>,
    size: usize,
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str(&self.to_string())
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
        use self::error::GridParseError::*;

        let size = size_from_string(s)?;
        let char_error = ::std::cell::Cell::new(false);
        let cells = s.lines().flat_map(|line| {
            line.as_bytes().iter().map(|c| match *c as char {
                '0' => Zero,
                '1' => One,
                '.' => Empty,
                _   => { char_error.set(true); Empty },
            })
        }).collect();
        if char_error.get() { return Err(UnexpectedCharacter) }
        Ok(Grid::from_parts(cells, size))
    }
}

/// Verifies that the grid encoded in the string is properly sized.
/// If successful, returns the size of the grid.
///
/// # Errors
///
/// Returns an error if the grid is not a square of non-nul, even size.
fn size_from_string(s: &str) -> Result<usize, GridSizeError> {
    use self::error::GridSizeError::*;

    if s.is_empty() { return Err(EmptyGrid) }
    let lines = s.lines().collect::<Vec<&str>>();
    let size = lines.len();
    if size & 1 == 1 { return Err(OddNumberSize) }
    for (i, line) in lines.iter().enumerate() {
        if line.chars().count() != size { return Err(NotASquare(i)) }
    }
    Ok(size)
}

impl Grid {
    /// Creates an new empty grid of a given size.
    ///
    /// # Errors
    ///
    /// Returns an error if the size is an odd number or 0.
    pub fn new(size: usize) -> Result<Grid, GridSizeError> {
        use self::error::GridSizeError::*;

        if size == 0 { Err(EmptyGrid) }
        else if size & 1 == 1 { Err(OddNumberSize) }
        else { Ok(Grid::from_parts(vec![Empty; size * size], size)) }
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
        self.cells.iter().all(|cell| cell.is_filled())
    }

    /// Verifies that the grid does not currently violate any of the rules.
    ///
    /// Returns `true` if the grid is legal.
    pub fn is_legal(&self) -> bool {
           self.check_rule1()
        && self.check_rule2()
        && self.check_rule3()
    }

    /// Verifies that a certain cell does not violate any of the rules.
    ///
    /// Returns `true` if the value is legal.
    pub fn is_cell_legal(&self, row: usize, col: usize) -> bool {
        self[(row, col)].is_empty() || {
            self.check_cell_rule1(row, col)
         && self.check_cell_rule2(row, col)
         && self.check_cell_rule3(row, col)
        }
    }

    /// Returns the index of the first `Empty` cell or `None` if the grid is filled.
    pub fn next_empty(&self) -> Option<(usize, usize)> {
        for (i, cell) in self.cells.iter().enumerate() {
            if cell.is_empty() {
                let row = i / self.size;
                let col = i % self.size;
                return Some((row, col))
            }
        }
        None
    }

    /// Solves the grid using both rules logic and a backtracking algorithm,
    /// and returns an array containing the solution(s).
    ///
    /// If no solution exists, an empty array is returned.
    ///
    /// # Errors
    ///
    /// Returns an error before any attempt at solving if
    /// the grid breaks any of the rules.
    ///
    /// Use the [`is_legal()`](#method.is_legal) method to know if the grid will trigger an `Err`.
    pub fn solve(&self) -> Result<Vec<Grid>, GridError> {
        if !self.is_legal() { return Err(GridError::Illegal) }
        let (mut stack, mut solutions) = (Vec::new(), Vec::new());
        let mut grid = self.clone();
        while grid.apply_rules() {}
        stack.push(grid);
        while !stack.is_empty() {
            let mut grid = stack.pop().unwrap();
            match grid.next_empty() {
                Some((row, col)) => {
                    grid[(row, col)] = One;
                    if grid.is_cell_legal(row, col) {
                        let mut grid = grid.clone();
                        while grid.apply_rules() {}
                        stack.push(grid);
                    }
                    grid[(row, col)] = Zero;
                    if grid.is_cell_legal(row, col) {
                        while grid.apply_rules() {}
                        stack.push(grid);
                    }
                },
                None => {
                    if grid.is_legal() { solutions.push(grid); }
                }
            }
        }
        Ok(solutions)
    }

    /// Suitable for printing in terminals.
    ///
    /// Encodes the grid as a printable string containing ANSI escape codes.
    /// The grid is compared to a reference grid.
    /// The cells that differ from the reference will be displayed in color.
    ///
    /// # Warning
    ///
    /// A red-colored cell signals that a `0` or a `1` from the reference grid
    /// was overwritten. (Which, if `reference` is the original grid
    /// and `self` is a solution, should _never_ happen.)
    pub fn to_string_diff(&self, reference: &Grid) -> String {
        let size = self.size;
        let mut buffer = String::with_capacity(size * (size * 10 + 1));
        buffer.extend(self.cells.chunks(size).zip(reference.cells.chunks(size)).flat_map(|(row, row_ref)| {
            row.iter().zip(row_ref.iter()).map(|(cell, cell_ref)| {
                match *cell {
                    Zero => {
                        // No color if nothing changed.
                        if cell == cell_ref { "0" }
                        // Color for 0 if we filled in a blank.
                        else if cell_ref.is_empty() { blue!('0') }
                        // Red for error if we overwrote.
                        else { red!('0') }
                    },
                    One => {
                        // No color if nothing changed.
                        if cell == cell_ref { "1" }
                        // Color for 1 if we filled in a blank.
                        else if cell_ref.is_empty() { yellow!('1') }
                        // Red for error if we overwrote.
                        else { red!('1') }
                    },
                    Empty => {
                        // No color if nothing changed.
                        if cell == cell_ref { "." }
                        // Red for error if we overwrote.
                        else { red!('.') }
                    }
                }
            }).chain(::std::iter::once("\n"))}));
        buffer
    }
}

impl Grid {
    /// Creates a `Grid` from an owned array of `Cell`s
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
        assert_ne!(size, 0);
        assert_eq!(size & 1, 0);
        assert_eq!(cells.len(), size * size);
        Grid {
            cells: cells,
            size: size,
        }
    }

    /// Encodes the grid to a string.
    ///
    /// Provides `Display` and `ToString`.
    fn to_string(&self) -> String {
        let size = self.size;
        let mut buffer = String::with_capacity(size * (size + 1));
        buffer.extend(self.cells.chunks(size).flat_map(|row| {
            row.iter().map(|cell| {
                match *cell {
                    Zero  => '0',
                    One   => '1',
                    Empty => '.',
                }
            }).chain(::std::iter::once('\n'))
        }));
        buffer
    }

    /// Verifies that the grid abides by rule 1.
    ///
    /// Rule 1: no more than two of either number adjacent to each other
    /// (both vertically and horizontally).
    fn check_rule1(&self) -> bool {
        for row in self.cells.chunks(self.size) {
            for triplet in row.windows(3) {
                if triplet[0].is_filled()
                && triplet[0] == triplet[1]
                && triplet[0] == triplet[2] { return false }
            }
        }
        for i in 0..self.size -2 {
            for j in 0..self.size {
                let cell = self[(i, j)];
                if cell.is_filled()
                && cell == self[(i+1, j)]
                && cell == self[(i+2, j)] { return false }
            }
        }
        true
    }

    /// Verifies that the grid abides by rule 2.
    ///
    /// Rule 2: each row and each column should contain an equal number of 0s and 1s.
    fn check_rule2(&self) -> bool {
        let nmax = self.size / 2;
        for row in self.cells.chunks(self.size) {
            let count = row.iter().fold((0, 0), |mut count, cell| {
                match *cell {
                    Zero  => count.0 += 1,
                    One   => count.1 += 1,
                    Empty => {},
                }
                count
            });
            if count.0 > nmax || count.1 > nmax { return false }
        }
        for i in 0..self.size {
            let mut count = (0, 0);
            for j in 0..self.size {
                match self[(j, i)] {
                    Zero  => count.0 += 1,
                    One   => count.1 += 1,
                    Empty => {},
                }
            }
            if count.0 > nmax || count.1 > nmax { return false }
        }
        true
    }

    /// Verifies that the grid abides by rule 3.
    ///
    /// Rule 3: no two rows and no two columns can be the same.
    fn check_rule3(&self) -> bool {
        for i in 0..self.size - 1 {
            for j in i + 1..self.size {
                if (0..self.size).all(|k| self[(i, k)].is_filled() && self[(i, k)] == self[(j, k)]) { return false }
                if (0..self.size).all(|k| self[(k, i)].is_filled() && self[(k, i)] == self[(k, j)]) { return false }
            }
        }
        true
    }

    /// Verifies that the cell with the given coordinates abides by rule 1.
    ///
    /// Rule 1: no more than two of either number adjacent to each other
    /// (both vertically and horizontally).
    fn check_cell_rule1(&self, row: usize, col: usize) -> bool {
        use std::cmp::min;

        for i in row.saturating_sub(2)..min(row + 1, self.size - 2) {
            let cell = self[(i, col)];
            if cell.is_filled()
            && cell == self[(i+1, col)]
            && cell == self[(i+2, col)] { return false }
        }
        for j in col.saturating_sub(2)..min(col + 1, self.size - 2) {
            let cell = self[(row, j)];
            if cell.is_filled()
            && cell == self[(row, j+1)]
            && cell == self[(row, j+2)] { return false }
        }
        true
    }

    /// Verifies that the cell with the given coordinates abides by rule 2.
    ///
    /// Rule 2: each row and each column should contain an equal number of 0s and 1s.
    fn check_cell_rule2(&self, row: usize, col: usize) -> bool {
        let nmax = self.size / 2;
        let mut count = (0, 0, 0, 0);
        for k in 0..self.size {
            match self[(row, k)] {
                Zero  => count.0 += 1,
                One   => count.1 += 1,
                Empty => {},
            }
            match self[(k, col)] {
                Zero  => count.2 += 1,
                One   => count.3 += 1,
                Empty => {},
            }
        }
        count.0 <= nmax && count.1 <= nmax && count.2 <= nmax && count.3 <= nmax
    }

    /// Verifies that the cell with the given coordinates abides by rule 3.
    ///
    /// Rule 3: no two rows and no two columns can be the same.
    fn check_cell_rule3(&self, row: usize, col: usize) -> bool {
        let rows_abide = (0..self.size).filter(|&i| i != row && self[(i, col)] == self[(row, col)])
                                       .map(|i| (0..self.size).all(|j| {
                                           self[(row, j)].is_filled() && self[(row, j)] == self[(i, j)]
                                       })).all(|b| !b);
        let cols_abide = (0..self.size).filter(|&j| j != col && self[(row, j)] == self[(row, col)])
                                       .map(|j| (0..self.size).all(|i| {
                                           self[(i, col)].is_filled() && self[(i, col)] == self[(i, j)]
                                       })).all(|b| !b);
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
           self.apply_rule1()
        || self.apply_rule2()
        || self.apply_rule3()
    }


    /// Disambiguates empty cells after rule 1.
    ///
    /// Rule 1: no more than two of either number adjacent to each other
    /// (both vertically and horizontally).
    fn apply_rule1(&mut self) -> bool {
        let mut rule_applied = false;
        for i in 0..self.size {
            for j in 0..self.size - 2 {
                let trio = (self[(i, j)], self[(i, j+1)], self[(i, j+2)]);
                match trio {
                    (Empty, Zero, Zero) => { self[(i, j  )] = One;  rule_applied = true; }
                    (Zero, Empty, Zero) => { self[(i, j+1)] = One;  rule_applied = true; }
                    (Zero, Zero, Empty) => { self[(i, j+2)] = One;  rule_applied = true; }
                    (Empty, One, One)   => { self[(i, j  )] = Zero; rule_applied = true; }
                    (One, Empty, One)   => { self[(i, j+1)] = Zero; rule_applied = true; }
                    (One, One, Empty)   => { self[(i, j+2)] = Zero; rule_applied = true; }
                    _ => {},
                }
                let trio = (self[(j, i)], self[(j + 1, i)], self[(j+2, i)]);
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
    /// Rule 2: each row and each column should contain an equal number of 0s and 1s.
    fn apply_rule2(&mut self) -> bool {
        let mut rule_applied = false;
        let nmax = self.size / 2;
        for i in 0..self.size {
            let mut count = (0, 0, 0, 0);
            for j in 0..self.size {
                match self[(i, j)] {
                    Zero  => count.0 += 1,
                    One   => count.1 += 1,
                    Empty => {},
                }
                match self[(j, i)] {
                    Zero  => count.2 += 1,
                    One   => count.3 += 1,
                    Empty => {},
                }
            }
            if count.0 == nmax && count.1 != nmax {
                rule_applied = true;
                for j in 0..self.size {
                    if self[(i, j)].is_empty() { self[(i, j)] = One; }
                }
            }
            else if count.1 == nmax && count.0 != nmax {
                rule_applied = true;
                for j in 0..self.size {
                    if self[(i, j)].is_empty() { self[(i, j)] = Zero; }
                }
            }
            if count.2 == nmax && count.3 != nmax {
                rule_applied = true;
                for j in 0..self.size {
                    if self[(j, i)].is_empty() { self[(j, i)] = One; }
                }
            }
            else if count.3 == nmax && count.2 != nmax {
                rule_applied = true;
                for j in 0..self.size {
                    if self[(j, i)].is_empty() { self[(j, i)] = Zero; }
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
            ($i: expr) => (self.cells[$i * self.size..($i + 1) * self.size])
        }

        let size = self.size;
        let mut rule_applied = false;
        for i in 0..size {
            if row!(i).iter().filter(|value| value.is_empty()).count() == 2 {
                for l in 0..size {
                    if l != i
                    && !row!(l).contains(&Empty)
                    && row!(i).iter().zip(row!(l).iter())
                                     .filter(|&(value, _)| value.is_filled())
                                     .all(|(value, other)| value == other) {
                                         for j in 0..size {
                                             if self[(i, j)].is_empty() {
                                                 self[(i, j)] = !self[(l, j)];
                                             }
                                         }
                                         rule_applied = true;
                                         break
                                     }
                }
            }
            let j = i;
            if (0..size).filter(|&l| self[(l, j)].is_empty()).count() == 2 {
                for m in 0..size {
                    if m != j
                    && (0..size).all(|i| self[(i, m)].is_filled())
                    && (0..size).filter(|&i| self[(i, j)].is_filled())
                                .all(|i| self[(i, j)] == self[(i, m)]) {
                                    for i in 0..size {
                                        if self[(i, j)].is_empty() {
                                            self[(i, j)] = !self[(i, m)];
                                }
                            }
                            rule_applied = true;
                            break
                        }
                }
            }
        }
        rule_applied
    }
}
