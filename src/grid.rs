use std::fmt::{Display, Write};

/// A container for a takuzu grid representation.
#[derive(Clone, Debug)]
pub struct Grid (Vec<Vec<Option<bool>>>);

impl Display for Grid {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        // Here to_string() is an inherent method,
        // not the ToString trait method
        f.write_str(&self.to_string())
    }
}

impl Grid {
    /// Solves the grid in place. Uses the rules to deduce the values of the empty cells
    /// when unambiguous. No recursive backtracking as of now (planned).
    pub fn solve(&mut self) {
        while self.apply_rules() {}
    }

    /// Creates a `Grid` from a preexisting array
    pub fn from_parts(array: Vec<Vec<Option<bool>>>) -> Grid {
        Grid(array)
    }

    /// Destroys a `Grid` and returns the underlying array
    pub fn as_parts(self) -> Vec<Vec<Option<bool>>> {
        self.0
    }

    /// Verifies that the grid is square and that the number of rows/columns is even.
    /// Returns an error message otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// if let Some(err) = grid_ref.check_size().err() {
    ///     println!("{}", err);
    /// }
    /// ```
    pub fn check_size(&self) -> Result<(), String> {
        let size = self.0.len();
        if size == 0 {
            return Err("empty grid".to_owned());
        }
        if size % 2 == 1 {
            return Err("odd number of rows".to_owned())
        }
        for (i, row) in self.0.iter().enumerate() {
            if row.len() != size {
                return Err(format!("line {}: not a square", i + 1));
            }
        }
        Ok(())
    }

    /// Verifies that the grid does not currently violate any of the rules.
    /// Returns `true` if the grid is legal.
    ///
    /// # Panics
    ///
    /// Panics if grid is not a square.
    ///
    pub fn check_rules(&self) -> bool {
        self.check_rule1()
            && self.check_rule2()
            && self.check_rule3()
    }

    /// Skims through the grid once for each rule and fills in the blanks
    /// where the value is unambiguous.
    /// Returns `true` if the grid was modified.
    ///
    /// # Panics
    ///
    /// Panics if grid is not a square.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut grid = Grid::read(&mut std::io::stdin()).unwrap();
    /// while grid.apply_rules() {}
    /// println("{}", grid);
    /// ```
    pub fn apply_rules(&mut self) -> bool {
        self.apply_rule1()
            || self.apply_rule2()
            || self.apply_rule3()
    }

    /// Returns `true` if the grid contains no blank.
    pub fn is_filled(&self) -> bool {
        self.0.iter().all(|row| row.iter().all(|tile| tile.is_some()))
    }

    /// Writes the grid to string.
    pub fn to_string(&self) -> String {
        let mut buffer = String::with_capacity(self.0.len() * (self.0.len() + 1));
        for row in self.0.iter() {
            for tile in row.iter() {
                match *tile {
                    Some(true) => { buffer.push('1'); }
                    Some(false) => { buffer.push('0'); }
                    None => { buffer.push('.'); }
                }
            }
            buffer.push('\n');
        }
        buffer
    }

    /// Suitable for terminals.
    ///
    /// Writes the grid to a string (containing escape characters).
    /// The grid is compared to the provided reference grid.
    /// The cells that differ from the reference will be displayed in color.
    pub fn to_string_pretty(&self, grid_ref: &Grid) -> String {
        let mut buffer = String::with_capacity(self.0.len() * (self.0.len() * 10 + 1));
        for (row, row_ref) in self.0.iter().zip(grid_ref.0.iter()) {
            for (tile, tile_ref) in row.iter().zip(row_ref.iter()) {
                match *tile {
                    Some(true) => {
                        if tile == tile_ref { buffer.push('1'); }
                        else { write!(&mut buffer, "\u{1b}[31m1\u{1b}[0m").unwrap(); }
                    },
                    Some(false) => {
                        if tile == tile_ref { buffer.push('0'); }
                        else { write!(&mut buffer, "\u{1b}[34m0\u{1b}[0m").unwrap(); }
                    },
                    None => { buffer.push('.'); }
                }
            }
            buffer.push('\n');
        }
        buffer
    }

    /// Prints the grid to stdout.
    pub fn print(&self) {
        print!("{}", self.to_string());
    }

    /// Prints the grid to stdout using the pretty format if appropriate.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let grid = Grid::read(&mut io::stdin()).unwrap();
    /// let grid_solved = my_solver(&grid);
    /// grid_solved.print_pretty(&grid);
    /// ```
    pub fn print_pretty(&self, grid_ref: &Grid) {
        let grid_str = {
            if ::isatty_stdout() {
                self.to_string_pretty(grid_ref)
            }
            else {
                self.to_string()
            }
        };
        print!("{}", grid_str);
    }
}

impl Grid {
    fn apply_rule1(&mut self) -> bool {
        // You can't put more than two identical numbers next to each other in a line
        let mut rule_applied = false;
        for i in 0..self.0.len() {
            for j in 0..self.0.len() - 2 {
                { // horizontal
                    let trio = &mut self.0[i][j..j + 3];
                    match trio {
                        [None, Some(a), Some(b)] if a == b => { trio[0] = Some(!a); rule_applied = true; },
                        [Some(a), None, Some(b)] if a == b => { trio[1] = Some(!a); rule_applied = true; },
                        [Some(a), Some(b), None] if a == b => { trio[2] = Some(!a); rule_applied = true; },
                        _ => {},
                    }
                }
                { // vertical
                    let trio = [self.0[j][i], self.0[j + 1][i], self.0[j + 2][i]];
                    match trio {
                        [None, Some(a), Some(b)] if a == b => { self.0[j    ][i] = Some(!a); rule_applied = true; },
                        [Some(a), None, Some(b)] if a == b => { self.0[j + 1][i] = Some(!a); rule_applied = true; },
                        [Some(a), Some(b), None] if a == b => { self.0[j + 2][i] = Some(!a); rule_applied = true; },
                        _ => {},
                    }
                }
            }
        }
        rule_applied
    }

    fn apply_rule2(&mut self) -> bool {
        // The number of 1s and 0s on each row and column must match
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

    fn apply_rule3(&mut self) -> bool {
        // You can't have two identical rows or two identical columns
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

    fn check_rule1(&self) -> bool {
        // You can't put more than two identical numbers next to each other in a line
        for i in 0..self.0.len() {
            for j in 0..self.0.len() - 2 {
                if self.0[i][j].is_some()
                    && self.0[i][j] == self.0[i][j + 1 ]
                    && self.0[i][j] == self.0[i][j + 2] { println!("rule 1"); return false }
                if self.0[j][i].is_some()
                    && self.0[j][i] == self.0[j + 1][i]
                    && self.0[j][i] == self.0[j + 2][i] { println!("rule 1"); return false }
            }
        }
        true
    }

    fn check_rule2(&self) -> bool {
        // The number of 1s and 0s on each row and column must match
        let nmax = self.0.len() / 2;
        for i in 0..self.0.len() {
            let (mut nh, mut nv) = ([0; 2], [0; 2]);
            for j in 0..self.0.len() {
                if let Some(a) = self.0[i][j] { if a { nh[1] += 1; } else { nh[0] += 1; } }
                if let Some(a) = self.0[j][i] { if a { nv[1] += 1; } else { nv[0] += 1; } }
            }
            if nh[0] > nmax || nh[1] > nmax
                || nv[0] > nmax || nv[1] > nmax { println!("rule 2"); return false }
        }
        true
    }

    fn check_rule3(&self) -> bool {
        // You can't have two identical rows or two identical columns
        for i in 0..self.0.len() - 1 {
            for j in i + 1..self.0.len() {
                if (0..self.0.len()).all(|k| self.0[i][k] != None && self.0[i][k] == self.0[j][k]) { println!("rule 3"); return false }
                if (0..self.0.len()).all(|k| self.0[k][i] != None && self.0[k][i] == self.0[k][j]) { println!("rule 3"); return false }
            }
        }
        true
    }
}
