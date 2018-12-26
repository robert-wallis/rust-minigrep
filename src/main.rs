// Copyright (C) 2018 Robert A. Wallis, all rights reserved.

use minigrep::*;
use std::env;

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

/// Checks the program args, as search paramaters.  And then runs a search on the file.
fn run() -> Result<(), GrepError> {
    let args: Vec<String> = env::args().collect();
    let params = SearchParams::from_args(&args)?;
    let contents = read_file(&params)?;
    for result in search(&params, &contents) {
        println!("{}", result);
    }
    Ok(())
}
