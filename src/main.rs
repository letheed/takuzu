#![warn(rust_2018_idioms)]

use anyhow::Error;
use std::{
    fmt::{self, Display},
    io::Read,
};
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
    let args = {
        let mut args = args;
        if args.is_empty() {
            args.push("-".into());
        }
        args
    };
    let filename = &args[0];
    run(filename);
    for filename in &args[1..] {
        println!();
        run(filename);
    }
}

fn run(filename: &str) {
    match solve_file(filename) {
        Ok((grid, solutions)) => print_solutions(&grid, &solutions, filename),
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
fn print_solutions(grid: &Grid, solutions: &[Grid], mut filename: &str) {
    if filename == "-" {
        filename = "(stdin)";
    }
    if solutions.is_empty() {
        println!("{}: no solution", filename);
        return;
    }
    let mut display = if isatty_stdout() {
        GridDisplay::GridDiff { reference: &grid, grid: &solutions[0] }
    } else {
        GridDisplay::Grid { grid: &solutions[0] }
    };
    if solutions.len() == 1 {
        print!("{}\n{}", filename, display);
    } else {
        print!("{}: 1\n{}", filename, display);
        for (i, solution) in solutions.iter().enumerate().skip(1) {
            print!("\n{}: {}\n{}", filename, i + 1, display.grid(solution));
        }
    }
}

/// Dynamic dispatch for the display implementation of the grid.
///
/// Displays the grid or the colored diff between the grid and the reference.
enum GridDisplay<'a> {
    Grid { grid: &'a Grid },
    GridDiff { reference: &'a Grid, grid: &'a Grid },
}

/// Displays the grid or the colored diff between the grid and the reference.
impl<'a> Display for GridDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            GridDisplay::Grid { grid } => grid.fmt(f),
            GridDisplay::GridDiff { reference, grid } => ANSIGridDiff(reference, grid).fmt(f),
        }
    }
}

/// Setter for the grid to be displayed.
///
/// Returns a reference to self for convenience so it can be used inline in a
/// print statement.
impl<'a> GridDisplay<'a> {
    fn grid(&mut self, grid: &'a Grid) -> &Self {
        match self {
            GridDisplay::Grid { grid: g } => *g = grid,
            GridDisplay::GridDiff { grid: g, .. } => *g = grid,
        }
        &*self
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
