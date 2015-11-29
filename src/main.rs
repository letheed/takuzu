/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! A Takuzu (a.k.a. Binairo) solver.
//!
//! # Usage
//!
//! ```shell
//! tackle [FILE]...
//! tackle {--help | --version}
//! ```
//!
//! If no `FILE` is provided, or if `FILE` is '`-`', reads from standard input.

extern crate libc;
extern crate takuzu;

use std::fs::File;
use std::io::{stderr, stdin, Write};
use takuzu::{Grid, Source};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const USAGE_STRING: &'static str =
r#"Usage: tackle [FILE]...
       tackle {--help | --version}

If no FILE is provided, or if FILE is "-", read from standard input.

Options:
    --help       display this message and exit
    --version    display the version and exit
"#;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.contains(&"--help".to_owned()) {
        print!("{}", USAGE_STRING);
        return
    }
    if args.contains(&"--version".to_owned()) {
        println!("takle (takuzu) {}", VERSION);
        return
    }
    if args.len() == 0 { solve_from(&mut stdin()); }
    else {
        takle(&args[0], args.len() > 1);
        for filename in args.iter().skip(1) {
            print!("\n");
            takle(filename, true);
        }
    }
}

/// Opens a file (or stdin) and feeds
/// the source to the solving + printing function.
fn takle(filename: &String, print_filename: bool) {
    if filename == "-" {
        if print_filename { println!("-"); }
        solve_from(&mut stdin());
    }
    else {
        let mut file = match File::open(filename) {
            Ok(file) => file,
            Err(err) => {
                write!(stderr(), "\"{}\": {}\n", filename, err).unwrap();
                return
            }
        };
        if print_filename { println!("{}", filename); }
        solve_from(&mut file);
    }
}

/// Reads a grid from a source, triggers the solving algorithm
/// and prints the solutions.
pub fn solve_from<T: Source + ?Sized>(source: &mut T) {
    let grid = match source.source() {
        Ok(grid) => grid,
        Err(err) => {
            write!(stderr(), "error: {}\n", err).unwrap();
            return
        },
    };
    let solutions = grid.solve();
    print_solutions(&grid, &solutions);
}

/// Prints the solution(s) to a grid on `stdout`.
///
/// Prints the grids with colors if appropriate (if `stdout` is a terminal).
/// If there is more than one solution, the grids are separated by
/// an empty line and preceded by a numbered label.
fn print_solutions(grid: &Grid, solutions: &Vec<Grid>) {
    if solutions.len() == 0 { write!(stderr(), "no solution\n").unwrap(); }
    else if solutions.len() == 1 {
        if isatty_stdout() { print!("{}", solutions[0].to_string_diff(&grid)); }
        else { print!("{}", solutions[0]); }
    }
    else {
        if isatty_stdout() {
            println!("solution 1");
            print!("{}", solutions[0].to_string_diff(&grid));
            for (i, sol) in solutions.into_iter().enumerate().skip(1) {
                println!("\nsolution {}", i + 1);
                print!("{}", sol.to_string_diff(&grid));
            }
        }
        else {
            print!("{}", solutions[0]);
            for sol in solutions.into_iter().skip(1) { print!("\n{}", sol); }
        }
    }
}

/// Returns `true` if `stdout` is a terminal.
pub fn isatty_stdout() -> bool {
    match unsafe { libc::isatty(libc::STDOUT_FILENO) } {
        1 => true,
        _ => false,
    }
}
