// Copyright (C) 2018 Robert A. Wallis, all rights reserved.

use std::error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum GrepError {
    /// program needs more agurments to run correctly
    NotEnoughParams,
    /// the OS returned a filesystem error
    IOError(String, io::Error),
}

/// Makes GrepError compatable with rust errors, so a function can use dyn Error type.
impl error::Error for GrepError {}

/// Used by print to turn GrepError enum into a string.
impl fmt::Display for GrepError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GrepError::NotEnoughParams => write!(f, "usage: minigrep [-i] keywords filename"),
            GrepError::IOError(filename, err) => match err.kind() {
                io::ErrorKind::NotFound => write!(f, "File {} not found.", filename),
                _ => write!(f, "{}", err),
            },
        }
    }
}
