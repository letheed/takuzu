//! A Takuzu (a.k.a. Binairo) solver.
//!
//! # Usage
//!
//! ```shell
//! tackle [FILE]...
//! ```
//!
//! If no `FILE` is provided, or if `FILE` is '`-`', will read from standard input.

extern crate takuzu;

use std::fs::File;
use std::io::{stderr, stdin, Write};
use takuzu::solve_from;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() == 0 {
        solve_from(&mut stdin());
    }
    for filename in args {
        if filename == "-" {
            solve_from(&mut stdin());
        }
        else {
            let mut file = match File::open(&filename) {
                Ok(file) => file,
                Err(err) => {
                    write!(stderr(), "\"{}\": {}\n", filename, err).unwrap();
                    continue
                }
            };
            solve_from(&mut file);
        }
    }
}
