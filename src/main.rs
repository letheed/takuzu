#![warn(rust_2018_idioms)]

use std::{
    fmt::{self, Display},
    io::Read,
};

use anyhow::Error;
use takuzu::{ANSIGridDiff, Grid};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const USAGE_STRING: &str = "\
Usage: takuzu [FILE]...
       takuzu {--help | --version}

If no FILE is provided, or if FILE is '-', read from standard input.

Options:
    --help       display this message and exit
    --version    display the version and exit
";

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.iter().any(|s| s == "--help") {
        print!("{}", USAGE_STRING);
        return;
    }
    if args.iter().any(|s| s == "--version") {
        println!("takuzu {}", VERSION);
        return;
    }
    if args.iter().filter(|&s| s == "-").count() > 1 {
        eprintln!("error: '-' (stdin) must not be mentionned more than once");
        return;
    }
    if args.is_empty() {
        run("-");
    } else {
        run(&args[0]);
        for filename in &args[1..] {
            println!();
            run(filename);
        }
    }
}

fn run(filename: &str) {
    match solve_file(filename) {
        Ok((grid, solutions)) => print_solutions(filename, &grid, &solutions),
        Err(err) => eprintln!("error: {}{}", filename, DisplayCauses(err)),
    }
}

/// Reads a file, parses it into a grid and returns that grid with its
/// solutions.
fn solve_file(filename: &str) -> Result<(Grid, Vec<Grid>), Error> {
    let grid: Grid = read_to_string(filename)?.parse()?;
    let solutions = grid.solve()?;
    Ok((grid, solutions))
}

/// Reads the contents of a file into a string,
/// or reads from `stdin` if filename is "-".
fn read_to_string(filename: &str) -> std::io::Result<String> {
    match filename {
        "-" => {
            let mut buffer = String::new();
            std::io::stdin().read_to_string(&mut buffer)?;
            Ok(buffer)
        }
        _ => std::fs::read_to_string(filename),
    }
}

/// Prints a grid's solution(s) to `stdout`.
///
/// If `stdout` is a terminal, prints the grids with colors highlighting the
/// differences with the unsolved original grid.
fn print_solutions(mut filename: &str, grid: &Grid, solutions: &[Grid]) {
    if filename == "-" {
        filename = "(stdin)";
    }
    if isatty_stdout() {
        print_loop(filename, solutions, |solution| ANSIGridDiff(&grid, solution));
    } else {
        print_loop(filename, solutions, |solution| solution);
    };

    #[inline]
    fn print_loop<'a, D>(filename: &str, solutions: &'a [Grid], format: impl Fn(&'a Grid) -> D)
    where D: Display {
        match solutions {
            [] => println!("{}: no solution", filename),
            [solution] => print!("{}\n{}", filename, format(solution)),
            [solution, solutions @ ..] => {
                print!("{}: 1\n{}", filename, format(solution));
                for (i, solution) in solutions.iter().enumerate() {
                    print!("\n{}: {}\n{}", filename, i + 2, format(solution));
                }
            }
        }
    }
}

/// Displays the causes of an `Error` recursively.
struct DisplayCauses(Error);

impl Display for DisplayCauses {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for cause in self.0.chain() {
            write!(f, ": {}", cause)?;
        }
        Ok(())
    }
}

/// Returns `true` if `stdout` is a terminal.
fn isatty_stdout() -> bool {
    match unsafe { libc::isatty(libc::STDOUT_FILENO) } {
        1 => true,
        _ => false,
    }
}
