/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::cmp::min;
use std::fmt::Display;
use std::iter;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

use self::error::{GridError, GridParseError, GridSizeError};

pub mod error;

/// A raw takuzu grid representation.
pub type Array = Vec<Vec<Option<bool>>>;

/// A container for takuzu grid manipulation.
///
/// It provides the internal logic and other convenience functions.
/// To create a `Grid` you can:
///
/// * create an `Array` yourself and use `Grid::new(array)`.
/// * use the `FromStr` trait, e.g. by calling `.parse()` on a string.
/// * use the `Source` trait, i.e. by calling `.source()` on any `Read` implementor.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Grid(Array);

impl Display for Grid {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl FromStr for Grid {
    type Err = GridParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parse_error = false;
        let array = s.lines().map(|line| line.chars()
                                  .map(|c| match c {
                                      '0' => Some(false),
                                      '1' => Some(true),
                                      '.' => None,
                                      _ => { parse_error = true; None }
                                  }).collect())
                             .collect();
        if parse_error {
            return Err(GridParseError::UnexpectedCharacter)
        }
        Grid::new(array).map_err(|err| GridParseError::CreationError(err.0, err.1))
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = Option<bool>;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.0[index.0][index.1]
    }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.0[index.0][index.1]
    }
}

// Public methods
impl Grid {
    /// Creates a `Grid` from a preexisting array.
    ///
    /// # Failure
    ///
    /// Returns an error enum and the invalid array if the grid
    /// is not a square of non-nul, even size or if the grid is illegal.
    pub fn new(array: Array) -> Result<Grid, (GridError, Array)> {
        let grid = Grid(array);
        if let Err(err) = grid.check_size() {
            return Err((GridError::BadSize(err), grid.0))
        }
        if !grid.is_legal() {
            return Err((GridError::Illegal, grid.0))
        }
        Ok(grid)
    }

    /// Returns the size of the grid.
    pub fn size(&self) -> usize {
        self.0.len()
    }

    /// Consumes a `Grid` and returns the underlying array.
    pub fn into_inner(self) -> Array {
        self.0
    }

    /// Returns `true` if the grid contains no empty cell.
    pub fn is_filled(&self) -> bool {
        self.0.iter().all(|row| row.iter().all(|cell| cell.is_some()))
    }

    /// Verifies that the grid does not currently violate any of the rules.
    /// Returns `true` if the grid is legal.
    pub fn is_legal(&self) -> bool {
        self.check_rule1()
            && self.check_rule2()
            && self.check_rule3()
    }

    /// Verifies that a certain cell does not violate any of the rules.
    /// Returns `true` if the value is legal.
    pub fn is_cell_legal(&self, row: usize, col: usize) -> bool {
        self.0[row][col].is_some()
            && self.check_cell_rule1(row, col)
            && self.check_cell_rule2(row, col)
            && self.check_cell_rule3(row, col)
    }

    /// Returns the index of the first empty cell or None if the grid is filled.
    pub fn next_empty(&self) -> Option<(usize, usize)> {
        for i in 0..self.0.len() {
            for j in 0..self.0.len() {
                if self.0[i][j] == None { return Some((i, j)) }
            }
        }
        None
    }

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
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::io;
    /// # use takuzu::Source;
    /// let mut grid = io::stdin().source().unwrap();
    /// while grid.apply_rules() {}
    /// println!("{}", grid);
    /// ```
    pub fn apply_rules(&mut self) -> bool {
        self.apply_rule1()
            || self.apply_rule2()
            || self.apply_rule3()
    }

    /// Solves the grid using both rules logic and a backtracking algorithm,
    /// and returns an array containing the solution(s).
    /// If no solution exists, an empty array is returned.
    pub fn solve(&self) -> Vec<Grid> {
        let (mut stack, mut solutions) = (Vec::new(), Vec::new());
        let mut grid = self.clone();
        while grid.apply_rules() {}
        stack.push(grid);
        while stack.len() != 0 {
            let mut grid = stack.pop().unwrap();
            match grid.next_empty() {
                Some((row, col)) => {
                    grid[(row, col)] = Some(true);
                    if grid.is_cell_legal(row, col) {
                        let mut grid = grid.clone();
                        while grid.apply_rules() {}
                        stack.push(grid);
                    }
                    grid[(row, col)] = Some(false);
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
        solutions
    }

    /// Suitable for terminals.
    ///
    /// Converts the grid to a printable string (containing escape characters).
    /// The grid is compared to a reference grid.
    /// The cells that differ from the reference will be displayed in color.
    ///
    /// A red-colored cell signals that a `0` or a `1` from the reference grid was overwritten.
    /// (Which, if `grid_ref` is the original grid and `self` is a solution, should *never* happen.)
    pub fn to_string_diff(&self, grid_ref: &Grid) -> String {
        let mut buffer = String::with_capacity(self.0.len() * (self.0.len() * 10 + 1));
        buffer.extend(self.0.iter().zip(grid_ref.0.iter()).flat_map(|(row, row_ref)| {
            row.iter().zip(row_ref.iter()).map(|(cell, cell_ref)| {
                match *cell {
                    Some(true) => {
                        // No color if nothing changed.
                        if cell == cell_ref { "1" }
                        // Color for 1 if we filled in a blank.
                        else if *cell_ref == None { "\u{1b}[33m1\u{1b}[0m" }
                        // Red for error if we overwrote a 0 (should never happen).
                        else { "\u{1b}[31m0\u{1b}[0m" }
                    },
                    Some(false) => {
                        // No color if nothing changed.
                        if cell == cell_ref { "0" }
                        // Color for 0 if we filled in a blank.
                        else if *cell_ref == None { "\u{1b}[34m0\u{1b}[0m" }
                        // Red for error if we overwrote a 1 (should never happen).
                        else { "\u{1b}[31m0\u{1b}[0m" }
                    },
                    None => {
                        // No color if nothing changed.
                        if cell == cell_ref { "." }
                        // Red for error if we overwrote a 0 or a 1 (should never happen).
                        else { "\u{1b}[31m0\u{1b}[0m" }
                    }
                }
            }).chain(iter::repeat("\n").take(1))}));
        buffer
    }
}

// Private methods
impl Grid {
    /// Verifies that the grid is square and that the number of rows/columns is even.
    /// Returns an error message otherwise.
    ///
    /// # Failure
    ///
    /// Returns an error string if the grid is not a square of non-nul, even size.
    fn check_size(&self) -> Result<(), GridSizeError> {
        let size = self.0.len();
        if size == 0 {
            return Err(GridSizeError::Empty)
        }
        if size % 2 == 1 {
            return Err(GridSizeError::OddRowNumber)
        }
        for (i, row) in self.0.iter().enumerate() {
            if row.len() != size {
                return Err(GridSizeError::NotASquare(i))
            }
        }
        Ok(())
    }

    /// Converts the grid to a string.
    fn to_string(&self) -> String {
        let mut buffer = String::with_capacity(self.0.len() * (self.0.len() + 1));
        buffer.extend(self.0.iter().flat_map(|row| row.iter().map(|cell| {
            match *cell {
                Some(true) => '1',
                Some(false) => '0',
                None => '.',
            }
        }).chain(iter::repeat('\n').take(1))));
        buffer
    }

    /// Verifies that the grid abides by rule 1.
    /// Rule 1: no more than two of either number adjacent to each other.
    /// (Both vertically and horizontally.)
    fn check_rule1(&self) -> bool {
        for i in 0..self.0.len() {
            for j in 0..self.0.len() - 2 {
                if self.0[i][j].is_some()
                    && self.0[i][j] == self.0[i][j+1]
                    && self.0[i][j] == self.0[i][j+2] { return false }
                if self.0[j][i].is_some()
                    && self.0[j][i] == self.0[j+1][i]
                    && self.0[j][i] == self.0[j+2][i] { return false }
            }
        }
        true
    }

    /// Verifies that the grid abides by rule 2.
    /// Rule 2: each row and each column should contain an equal number of 0s and 1s.
    fn check_rule2(&self) -> bool {
        let nmax = self.0.len() / 2;
        for i in 0..self.0.len() {
            let (mut nh, mut nv) = ([0; 2], [0; 2]);
            for j in 0..self.0.len() {
                if let Some(a) = self.0[i][j] { if a { nh[1] += 1; } else { nh[0] += 1; } }
                if let Some(a) = self.0[j][i] { if a { nv[1] += 1; } else { nv[0] += 1; } }
            }
            if nh[0] > nmax || nh[1] > nmax
                || nv[0] > nmax || nv[1] > nmax { return false }
        }
        true
    }

    /// Verifies that the grid abides by rule 3.
    /// Rule 3: no two rows and no two columns can be the same.
    fn check_rule3(&self) -> bool {
        for i in 0..self.0.len() - 1 {
            for j in i + 1..self.0.len() {
                if (0..self.0.len()).all(|k| self.0[i][k] != None && self.0[i][k] == self.0[j][k]) { return false }
                if (0..self.0.len()).all(|k| self.0[k][i] != None && self.0[k][i] == self.0[k][j]) { return false }
            }
        }
        true
    }

    /// Verifies that the cell abides by rule 1.
    /// Rule 1: no more than two of either number adjacent to each other.
    /// (Both vertically and horizontally.)
    fn check_cell_rule1(&self, row: usize, col: usize) -> bool {
        for i in row.saturating_sub(2)..min(row + 1, self.0.len() - 2) {
            if self.0[i][col].is_some()
                && self.0[i][col] == self.0[i+1][col]
                && self.0[i][col] == self.0[i+2][col] { return false }
        }
        for j in col.saturating_sub(2)..min(col + 1, self.0.len() - 2) {
            if self.0[row][j].is_some()
                && self.0[row][j] == self.0[row][j+1]
                && self.0[row][j] == self.0[row][j+2] { return false }
        }
        true
    }

    /// Verifies that the cell abides by rule 2.
    /// Rule 2: each row and each column should contain an equal number of 0s and 1s.
    fn check_cell_rule2(&self, row: usize, col: usize) -> bool {
        let (mut nh, mut nv) = ([0; 2], [0; 2]);
        for k in 0..self.0.len() {
            if let Some(a) = self.0[row][k] { if a { nh[1] += 1; } else { nh[0] += 1; } }
            if let Some(a) = self.0[k][col] { if a { nv[1] += 1; } else { nv[0] += 1; } }
        }
        let nmax = self.0.len() / 2;
        if nh[0] > nmax || nh[1] > nmax
            || nv[0] > nmax || nv[1] > nmax { return false }
        true
    }

    /// Verifies that the cell abides by rule 3.
    /// Rule 3: no two rows and no two columns can be the same.
    fn check_cell_rule3(&self, row: usize, col: usize) -> bool {
        (0..self.0.len()).filter(|&i| i != row && self.0[i][col] == self.0[row][col])
                         .map(|i| (0..self.0.len()).all(|j| self.0[row][j] != None && self.0[row][j] == self.0[i][j]))
                         .all(|b| !b)
            && (0..self.0.len()).filter(|&j| j != col && self.0[row][j] == self.0[row][col])
                                .map(|j| (0..self.0.len()).all(|i| self.0[i][col] != None && self.0[i][col] == self.0[i][j]))
                                .all(|b| !b)
    }

    /// Disambiguates blank cells after rule 1.
    /// Rule 1: no more than two of either number adjacent to each other.
    /// (Both vertically and horizontally.)
    fn apply_rule1(&mut self) -> bool {
        let mut rule_applied = false;
        for i in 0..self.0.len() {
            for j in 0..self.0.len() - 2 {
                { // horizontal
                    let trio = (self.0[i][j], self.0[i][j+1], self.0[i][j+2]);
                    match trio {
                        (None, Some(a), Some(b)) if a == b => { self.0[i][j  ] = Some(!a); rule_applied = true; }
                        (Some(a), None, Some(b)) if a == b => { self.0[i][j+1] = Some(!a); rule_applied = true; }
                        (Some(a), Some(b), None) if a == b => { self.0[i][j+2] = Some(!a); rule_applied = true; }
                        _ => {},
                    }
                }
                { // vertical
                    let trio = (self.0[j][i], self.0[j + 1][i], self.0[j+2][i]);
                    match trio {
                        (None, Some(a), Some(b)) if a == b => { self.0[j  ][i] = Some(!a); rule_applied = true; }
                        (Some(a), None, Some(b)) if a == b => { self.0[j+1][i] = Some(!a); rule_applied = true; }
                        (Some(a), Some(b), None) if a == b => { self.0[j+2][i] = Some(!a); rule_applied = true; }
                        _ => {},
                    }
                }
            }
        }
        rule_applied
    }

    /// Disambiguates blank cells after rule 2.
    /// Rule 2: each row and each column should contain an equal number of 0s and 1s.
    fn apply_rule2(&mut self) -> bool {
        let mut rule_applied = false;
        let nmax = self.0.len() / 2;
        for i in 0..self.0.len() {
            let (mut nh, mut nv) = ([0; 2], [0; 2]);
            for j in 0..self.0.len() {
                if let Some(a) = self.0[i][j] { if a { nh[1] += 1; } else { nh[0] += 1; } }
                if let Some(a) = self.0[j][i] { if a { nv[1] += 1; } else { nv[0] += 1; } }
            }
            if nh[0] == nmax && nh[1] != nh[0] {
                rule_applied = true;
                for j in 0..self.0.len() {
                    if self.0[i][j] == None { self.0[i][j] = Some(true); }
                }
            }
            else if nh[1] == nmax && nh[0] != nh[1] {
                rule_applied = true;
                for j in 0..self.0.len() {
                    if self.0[i][j] == None { self.0[i][j] = Some(false); }
                }
            }
            if nv[0] == nmax && nv[1] != nv[0] {
                rule_applied = true;
                for j in 0..self.0.len() {
                    if self.0[j][i] == None { self.0[j][i] = Some(true); }
                }
            }
            else if nv[1] == nmax && nv[0] != nv[1] {
                rule_applied = true;
                for j in 0..self.0.len() {
                    if self.0[j][i] == None { self.0[j][i] = Some(false); }
                }
            }
        }
        rule_applied
    }

    /// Disambiguates blank cells after rule 3.
    /// Rule 3: no two rows and no two columns can be the same.
    fn apply_rule3(&mut self) -> bool {
        let mut rule_applied = false;
        for i in 0..self.0.len() {
            if self.0[i].iter().filter(|&&value| value == None).count() == 2 {
                for j in 0..self.0.len() {
                    if j != i
                        && !self.0[j].contains(&None)
                        && self.0[i].iter().zip(self.0[j].iter()).filter(|&(&value, _)| value != None).all(|(value, other)| value == other) {
                            for k in 0..self.0.len() {
                                if self.0[i][k] == None {
                                    self.0[i][k] = Some(!self.0[j][k].unwrap());
                                }
                            }
                            rule_applied = true;
                            break
                        }
                }
            }
            if (0..self.0.len()).map(|j| self.0[j][i] == None).filter(|&b| b).count() == 2 {
                for j in 0..self.0.len() {
                    if j != i
                        && (0..self.0.len()).all(|k| self.0[k][j] != None)
                        && (0..self.0.len()).filter(|&k| self.0[k][i] != None).all(|k| self.0[k][i] == self.0[k][j]) {
                            for k in 0..self.0.len() {
                                if self.0[k][i] == None {
                                    self.0[k][i] = Some(!self.0[k][j].unwrap());
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
