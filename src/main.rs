// Copyright (C) 2018 Robert A. Wallis, all rights reserved.
use minigrep;

fn main() {
    if let Err(err) = minigrep::run() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
