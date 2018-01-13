/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

//! A Takuzu (a.k.a. Binairo) solver.
//!
//! # Usage
//!
//! ```shell
//! takuzu [FILE]...
//! takuzu {--help | --version}
//! ```
//!
//! If no `FILE` is provided, or if `FILE` is '`-`', reads from standard input.

extern crate failure;
extern crate libc;
extern crate takuzu;

use failure::Error;
use std::fmt::Write;
use std::io::stdin;
use takuzu::{Grid, Source};

static VERSION: &str = env!("CARGO_PKG_VERSION");
static USAGE_STRING: &str = "\
Usage: takuzu [FILE]...
       takuzu {--help | --version}

If no FILE is provided, or if FILE is '-', read from standard input.

Options:
    --help       display this message and exit
    --version    display the version and exit
";

macro_rules! handle_and_print {
    ($result:expr) => {
        match $result {
            Ok(ok) => print_solutions(&ok.0, &ok.1, None),
            Err(err) => eprintln!("error: {}", causes_fold(&err)),
        }
    };
    ($result:expr, $filename:expr) => {
        match $result {
            Ok(ok) => print_solutions(&ok.0, &ok.1, Some($filename)),
            Err(err) => eprintln!("error: \"{}\": {}", $filename, causes_fold(&err)),
        }
    };
    ($result:expr, $filename:expr, $print_filename:expr) => {
        if $print_filename {
            handle_and_print!($result, $filename)
        } else {
            handle_and_print!($result)
        }
    };
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut stdin_found = false;
    for arg in &args {
        if arg == "--help" {
            print!("{}", USAGE_STRING);
            return;
        } else if arg == "--version" {
            println!("takuzu {}", VERSION);
            return;
        } else if arg == "-" {
            if stdin_found {
                eprintln!("error: can't mention \"-\" (stdin) more than once");
                return;
            }
            stdin_found = true;
        }
    }
    if args.is_empty() {
        handle_and_print!(solve_from("-"));
    } else {
        handle_and_print!(solve_from(&args[0]), &args[0], args.len() > 1);
        for filename in args.iter().skip(1) {
            println!();
            handle_and_print!(solve_from(filename), filename);
        }
    }
}

/// Opens a file (or `stdin` if filename is "-"), reads a grid,
/// triggers the solving algorithm and returns the grid with its solutions.
fn solve_from(filename: &str) -> Result<(Grid, Vec<Grid>), Error> {
    let grid = match filename {
        "-" => stdin().source()?,
        _ => ::std::fs::File::open(filename)?.source()?,
    };
    let solutions = grid.solve()?;
    Ok((grid, solutions))
}

/// Prints a grid's solution(s) to `stdout`.
///
/// If there is more than one solution, the grids are separated an empty line.
/// If `stdout` is a terminal, prints the grids with colors and preceded by
/// a numbered label. Optionnally, prints the filename before the solutions.
fn print_solutions(grid: &Grid, solutions: &[Grid], filename: Option<&str>) {
    if solutions.is_empty() {
        if let Some(filename) = filename {
            eprint!("{}: ", filename);
        }
        eprintln!("no solution");
    } else {
        if let Some(filename) = filename {
            println!("{}", filename);
        }
        if isatty_stdout() {
            if solutions.len() == 1 {
                print!("{}", solutions[0].to_string_diff(grid));
            } else {
                println!("solution 1");
                print!("{}", solutions[0].to_string_diff(grid));
                for (i, solution) in solutions.iter().enumerate().skip(1) {
                    println!("\nsolution {}", i + 1);
                    print!("{}", solution.to_string_diff(grid));
                }
            }
        } else {
            print!("{}", solutions[0]);
            for sol in solutions.iter().skip(1) {
                print!("\n{}", sol);
            }
        }
    }
}

/// Returns a string containing all the causes of an `Error`.
fn causes_fold(error: &Error) -> String {
    error
        .causes()
        .skip(1)
        .fold(error.to_string(), |mut buffer, cause| {
            write!(&mut buffer, ": {}", cause).unwrap();
            buffer
        })
}

/// Returns `true` if `stdout` is a terminal.
fn isatty_stdout() -> bool {
    match unsafe { libc::isatty(libc::STDOUT_FILENO) } {
        1 => true,
        _ => false,
    }
}
