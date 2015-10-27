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

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn takle(filename: &String) {
    if filename == "-" {
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
        solve_from(&mut file);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.contains(&"--version".to_owned()) {
        println!("takle (takuzu) v{}", VERSION);
        return
    }
    if args.len() == 0 { solve_from(&mut stdin()); }
    else {
        if args.len() > 1 { println!("{}", &args[0]); }
        takle(&args[0]);
        for filename in args.iter().skip(1) {
            println!("\n{}", filename);
            takle(filename);
        }
    }
}
